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