//! Example of using a blocking Wifi with a DHCP configuration that has a user-supplied host name
//!
//! Add your own SSID and password for the access point
//!
//! Once the wifi is connected, the hostname will be set to "foo"
//! Try pinging it from your PC with `ping foo`

#![allow(unknown_lints)]
#![allow(unexpected_cfgs)]

use core::convert::TryInto;

use esp_idf_svc::wifi::{AuthMethod, ClientConfiguration, Configuration as WifiConfiguration};

use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::ipv4::{
    ClientConfiguration as IpClientConfiguration, Configuration as IpConfiguration,
    DHCPClientSettings,
};
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::netif::{EspNetif, NetifConfiguration, NetifStack};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi, WifiDriver};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use std::net::UdpSocket;

use log::info;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    const LEFT_BULB_IP: &str = env!("LEFT_BULB_IP");
    const RIGHT_BULB_IP: &str = env!("RIGHT_BULB_IP");

    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let wifi = WifiDriver::new(peripherals.modem, sys_loop.clone(), Some(nvs))?;
    let wifi = configure_wifi(wifi)?;

    let mut wifi = BlockingWifi::wrap(wifi, sys_loop)?;
    connect_wifi(&mut wifi)?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

    info!("Wifi Interface info: {ip_info:?}");

    turn_off(LEFT_BULB_IP).expect("Failed to turn off bulb");
    turn_on(RIGHT_BULB_IP).expect("Failed to turn on bulb");

    loop {
        std::thread::sleep(core::time::Duration::from_secs(5));
    }
}

fn configure_wifi(wifi: WifiDriver) -> anyhow::Result<EspWifi> {
    const SSID: &str = env!("WIFI_SSID");
    const PASSWORD: &str = env!("WIFI_PASS");

    let mut wifi = EspWifi::wrap_all(
        wifi,
        // Note that setting a custom hostname can be used with any network adapter, not just Wifi
        // I.e. that would work with Eth as well, because DHCP is an L3 protocol
        EspNetif::new_with_conf(&NetifConfiguration {
            ip_configuration: Some(IpConfiguration::Client(IpClientConfiguration::DHCP(
                DHCPClientSettings {
                    hostname: Some("foo".try_into().unwrap()),
                },
            ))),
            ..NetifConfiguration::wifi_default_client()
        })?,
        #[cfg(esp_idf_esp_wifi_softap_support)]
        EspNetif::new(NetifStack::Ap)?,
    )?;

    let wifi_configuration = WifiConfiguration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        channel: None,
        ..Default::default()
    });
    wifi.set_configuration(&wifi_configuration)?;

    Ok(wifi)
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    wifi.start()?;
    info!("Wifi started");

    wifi.connect()?;
    info!("Wifi connected");

    wifi.wait_netif_up()?;
    info!("Wifi netif up");

    Ok(())
}

fn turn_on(ip: &str) -> anyhow::Result<()> {
    let addr = format!("{}:38899", ip);
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    
    // JSON payload to turn the bulb on and set it to a warm white
    let msg = r#"{"method":"setPilot","params":{"state":true,"temp":2700, "dimming": 75}}"#;
    
    socket.send_to(msg.as_bytes(), addr)?;
    println!("Command sent to WiZ bulb at {}!", ip);

    Ok(())
}

fn turn_off(ip: &str) -> anyhow::Result<()> {
    let addr = format!("{}:38899", ip);
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    
    // JSON payload to turn the bulb on and set it to a warm white
    let msg = r#"{"method":"setPilot","params":{"state":false}}"#;
    
    socket.send_to(msg.as_bytes(), addr)?;
    println!("Command sent to WiZ bulb at {}!", ip);

    Ok(())
}
