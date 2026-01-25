#![no_std]

use core::str::FromStr;

use embassy_net::{IpAddress, IpEndpoint, Ipv4Address};
use embassy_net::{
    IpListenEndpoint,
    udp::{PacketMetadata, UdpSocket},
};
use embassy_time::{Duration, Timer};
use esp_println::println;

pub const SSID: &str = env!("WIFI_SSID");
pub const PASSWORD: &str = env!("WIFI_PASSWORD");

pub fn get_bulb_endpoint(ip: &str) -> IpEndpoint {
    let port: u16 = 38899;

    return IpEndpoint::new(
        IpAddress::Ipv4(Ipv4Address::from_str(ip).expect("Invalid IP specified.")),
        port,
    );
}
pub const LEFT_BULB_IP: &str = env!("LEFT_BULB_IP");
pub const RIGHT_BULB_IP: &str = env!("RIGHT_BULB_IP");

pub async fn turn_off(stack: embassy_net::Stack<'_>) {
    // Initialize UDP socket
    let mut rx_meta = [PacketMetadata::EMPTY; 16];
    let mut rx_buffer = [0; 4096];
    let mut tx_meta = [PacketMetadata::EMPTY; 16];
    let mut tx_buffer = [0; 4096];
    let mut socket = UdpSocket::new(
        stack,
        &mut rx_meta,
        &mut rx_buffer,
        &mut tx_meta,
        &mut tx_buffer,
    );
    match socket.bind(IpListenEndpoint {
        addr: Option::None,
        port: 38900,
    }) {
        Ok(_) => println!("UDP socket bound."),
        Err(e) => println!("Failed to bind UDP socket: {e:?}"),
    };

    // JSON payload to turn the bulb off
    let msg = r#"{"method":"setPilot","params":{"state":false}}"#;
    match socket
        .send_to(msg.as_bytes(), get_bulb_endpoint(LEFT_BULB_IP))
        .await
    {
        Ok(_) => println!(
            "Command sent to bulb: {:?}.",
            get_bulb_endpoint(LEFT_BULB_IP)
        ),
        Err(e) => println!("Failed to send command to bulb: {e:?}"),
    }
    match socket
        .send_to(msg.as_bytes(), get_bulb_endpoint(RIGHT_BULB_IP))
        .await
    {
        Ok(_) => println!(
            "Command sent to bulb: {:?}.",
            get_bulb_endpoint(RIGHT_BULB_IP)
        ),
        Err(e) => println!("Failed to send command to bulb: {e:?}"),
    }
    Timer::after(Duration::from_millis(5_000)).await;
}

pub async fn turn_on(stack: embassy_net::Stack<'_>) {
    // Initialize UDP socket
    let mut rx_meta = [PacketMetadata::EMPTY; 16];
    let mut rx_buffer = [0; 4096];
    let mut tx_meta = [PacketMetadata::EMPTY; 16];
    let mut tx_buffer = [0; 4096];
    let mut socket = UdpSocket::new(
        stack,
        &mut rx_meta,
        &mut rx_buffer,
        &mut tx_meta,
        &mut tx_buffer,
    );
    match socket.bind(IpListenEndpoint {
        addr: Option::None,
        port: 38900,
    }) {
        Ok(_) => println!("UDP socket bound."),
        Err(e) => println!("Failed to bind UDP socket: {e:?}"),
    };

    // JSON payload to turn the bulb on and set it to a warm white
    let msg = r#"{"method":"setPilot","params":{"state":true,"temp":2700, "dimming": 75}}"#;
    match socket
        .send_to(msg.as_bytes(), get_bulb_endpoint(LEFT_BULB_IP))
        .await
    {
        Ok(_) => println!(
            "Command sent to bulb: {:?}.",
            get_bulb_endpoint(LEFT_BULB_IP)
        ),
        Err(e) => println!("Failed to send command to bulb: {e:?}"),
    }
    match socket
        .send_to(msg.as_bytes(), get_bulb_endpoint(RIGHT_BULB_IP))
        .await
    {
        Ok(_) => println!(
            "Command sent to bulb: {:?}.",
            get_bulb_endpoint(RIGHT_BULB_IP)
        ),
        Err(e) => println!("Failed to send command to bulb: {e:?}"),
    }
    Timer::after(Duration::from_millis(5_000)).await;
}
