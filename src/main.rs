#![feature(never_type)]
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use embedded_svc::wifi::Wifi;
use std::sync::Arc;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<!, Error> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();
    
    println!("Hello, world!");
    
    let std_in = std::io::stdin();
    let if_stack = Arc::from(esp_idf_svc::netif::EspNetifStack::new()?);
    let sys_loop_stack = Arc::from(esp_idf_svc::sysloop::EspSysLoopStack::new()?);
    let nvs = Arc::from(esp_idf_svc::nvs::EspDefaultNvs::new()?);

    let mut wifi = esp_idf_svc::wifi::EspWifi::new(if_stack, sys_loop_stack, nvs)?;

    println!("Scanning for access points...");
    let ap_infos = wifi.scan()?;

    println!("Here they are:");
    for ap_info in ap_infos.into_iter() {
        println!("{:#?}", ap_info);
    }

    loop {
        println!("SSID?");
        let mut ssid = String::default();
        std_in.read_line(&mut ssid).expect("Failed to read from stdin");
        println!("Password?");
        let mut password = String::default();
        std_in.read_line(&mut password).expect("Failed to read from stdin");

        println!("Connecting...");
        wifi.set_configuration(&embedded_svc::wifi::Configuration::Client(embedded_svc::wifi::ClientConfiguration {
            ssid,
            password,
            ..Default::default()
        }))?;
        match wifi.get_status() {
            embedded_svc::wifi::Status(embedded_svc::wifi::ClientStatus::Started(status), _) => {
                println!("Connected!");
                println!("{:?}", status);
                break;
            },
            _ => {
                println!("Failed to connect.");
            },
        }
    }

    loop {

        std::thread::sleep(core::time::Duration::from_secs(5));
    }
}
