use dotenvy::dotenv;

fn main() {
    dotenv().ok();
    for variable in dotenvy::from_path_iter(".env").unwrap() {
        let (key, value) = variable.unwrap();
        println!("cargo:rustc-env={key}={value}");
    }
    println!("cargo:rerun-if-changed=.env");

    embuild::espidf::sysenv::output();
}
