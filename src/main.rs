#![feature(never_type)]
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use embedded_svc::wifi::Wifi;
use esp_idf_svc::{
    netif::EspNetifStack,
    nvs::EspDefaultNvs,
    sysloop::EspSysLoopStack,
    wifi::EspWifi,
};
use std::sync::Arc;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<!, Error> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();
    
    println!("Hello, world!");
    
    let if_stack = Arc::from(EspNetifStack::new()?);
    let sys_loop_stack = Arc::from(EspSysLoopStack::new()?);
    let nvs = Arc::from(EspDefaultNvs::new()?);

    let mut wifi = EspWifi::new(if_stack, sys_loop_stack, nvs)?;

    loop {
        println!("Scanning for access points...");
        let ap_infos = wifi.scan()?;

        println!("Here they are:");
        for ap_info in ap_infos.into_iter() {
            println!("{:#?}", ap_info);
        }

        std::thread::sleep(core::time::Duration::from_secs(5));
    }
}
