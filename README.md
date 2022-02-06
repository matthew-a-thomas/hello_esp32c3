# hello_esp32c3

Rust on [ESP32-C3-DevKitM-1](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/hw-reference/esp32c3/user-guide-devkitm-1.html).

## Behavior

This application repeatedly scans for WiFi access points.

## Setup

Install Clang:<br/>
https://clang.llvm.org/get_started.html

Install Python >=3.7:<br/>
https://www.python.org/downloads/

Install Rust:<br/>
https://rustup.rs/

Install Rust components
```
rustup install nightly
rustup component add rust-src --toolchain nightly
cargo install cargo-generate ldproxy espflash espmonitor
```

## Build

```
cargo build
```

## Flash

```
espflash PORT target/riscv32imc-esp-espidf/debug/hello_esp32c3
```

Where _PORT_ is the port (e.g. COM1 on Windows)

## Monitor

```
espmonitor PORT
```

Where _PORT_ is the port (e.g. COM1 on Windows)

## More info

https://github.com/esp-rs

https://github.com/esp-rs/esp-idf-template

https://github.com/ivmarkov/rust-esp32-std-demo
