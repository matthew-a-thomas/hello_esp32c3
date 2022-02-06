#![feature(never_type)]
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use embedded_svc::wifi::Wifi;
use std::io::BufRead;
use std::io::Read;
use std::sync::Arc;

type Error = Box<dyn std::error::Error>;

fn connect_wifi(
    if_stack: Arc<esp_idf_svc::netif::EspNetifStack>,
    sys_loop_stack: Arc<esp_idf_svc::sysloop::EspSysLoopStack>,
    nvs: Arc<esp_idf_svc::nvs::EspDefaultNvs>,
    std_in: &mut impl BufRead,
) -> Result<esp_idf_svc::wifi::EspWifi, Error> {
    loop {
        let mut wifi = esp_idf_svc::wifi::EspWifi::new(if_stack.clone(), sys_loop_stack.clone(), nvs.clone())?;

        println!("SSID?");
        let mut ssid = String::default();
        std_in.read_line(&mut ssid)?;
        let ssid = ssid.trim().into();
        println!("Password?");
        let mut password = String::default();
        std_in.read_line(&mut password)?;
        let password = password.trim().into();

        println!("Connecting to \"{}\"...", &ssid);
        match wifi.set_configuration(&embedded_svc::wifi::Configuration::None) {
            Ok(()) => {},
            Err(err) => {
                eprintln!("{}", err);
                continue;
            },
        };
        match wifi.set_configuration(&embedded_svc::wifi::Configuration::Client(embedded_svc::wifi::ClientConfiguration {
            ssid,
            password,
            ..Default::default()
        })) {
            Ok(()) => {},
            Err(err) => {
                eprintln!("{}", err);
                continue;
            },
        };
        match wifi.get_status() {
            embedded_svc::wifi::Status(embedded_svc::wifi::ClientStatus::Started(status), _) => {
                println!("Connected!");
                println!("{:?}", status);
                return Ok(wifi);
            },
            _ => {
                eprintln!("Failed to connect.");
            },
        }

        std::thread::sleep(core::time::Duration::from_secs(5));
    }
}

fn main() -> Result<!, Error> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();
    
    println!("Hello, world!");
    
    let std_in = std::io::stdin();
    let std_in = std_in.lock();
    let std_in: BlockingReader<_> = std_in.into();
    let mut std_in = std::io::BufReader::new(std_in);
    let if_stack = Arc::from(esp_idf_svc::netif::EspNetifStack::new()?);
    let sys_loop_stack = Arc::from(esp_idf_svc::sysloop::EspSysLoopStack::new()?);
    let nvs = Arc::from(esp_idf_svc::nvs::EspDefaultNvs::new()?);

    {
        let mut wifi = esp_idf_svc::wifi::EspWifi::new(if_stack.clone(), sys_loop_stack.clone(), nvs.clone())?;

        println!("Scanning for access points...");
        let ap_infos = wifi.scan()?;

        println!("Here they are:");
        for ap_info in ap_infos.into_iter() {
            println!("{:#?}", ap_info);
        }
    }

    let _wifi = connect_wifi(if_stack.clone(), sys_loop_stack.clone(), nvs.clone(), &mut std_in)?;

    let sntp = esp_idf_svc::sntp::EspSntp::new_default()?;
    println!("Waiting for SNTP...");
    loop {
        let status = sntp.get_sync_status();
        if status == esp_idf_svc::sntp::SyncStatus::Completed {
            println!("Got SNTP!");
            break;
        }
        let now = std::time::SystemTime::now();
        let seconds = now.clone().duration_since(std::time::SystemTime::UNIX_EPOCH)?.as_secs();
        let minutes = seconds / 60;
        if minutes >= 5 {
            eprintln!("Done waiting for SNTP");
            break;
        }
        println!("{:?}\n{:?}", now, status);
        std::thread::sleep(core::time::Duration::from_secs(1));
    }
    loop {
        let now = std::time::SystemTime::now();
        let now = now.duration_since(std::time::SystemTime::UNIX_EPOCH)?;
        println!("{:?}", now);
        std::thread::sleep(core::time::Duration::from_secs(5));
    }
}

/// https://github.com/ivmarkov/rust-esp32-std-demo/issues/59
struct BlockingReader<R: std::io::Read> {
    poll: core::time::Duration,
    reader: R,
}

impl<R: Read> From<R> for BlockingReader<R> {
    fn from(reader: R) -> Self {
        Self {
            poll: core::time::Duration::from_millis(250),
            reader,
        }
    }
}

impl<R: Read> std::io::Read for BlockingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.len() == 0 {
            return Ok(0);
        }
        loop {
            match self.reader.read(buf) {
                Ok(num_bytes) => return Ok(num_bytes),
                Err(error) => match error.kind() {
                    std::io::ErrorKind::WouldBlock => std::thread::sleep(self.poll),
                    _ => return Err(error),
                }
            }
        }
    }
}
