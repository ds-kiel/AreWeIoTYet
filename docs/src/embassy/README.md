# Embassy

## Embassy & Softdevice
### Pure Embassy
1. At least for Ubuntu: Make sure installed are:
    1. pkg-config
    2. libudev-dev
    3. probe-run
    4. probe-rs-cli
2. Clone embassy & update submodules
3. Run embassy example:
    1. go to examples/nrf52840
    2. run: cargo run --bin blinky --release
4. Now the LED1 should be blinking

### embassy/nrf-softdevice
1. run: rustup update
2. Download S140 version 7.3.0
3. Get embassy/nrf-softdevice repo
4. Unzip content of s140_nrf52_7.3.0 into nrf-softdevice/examples
5. run: probe-rs-cli erase --chip nrf52840
6. run: probe-rs-cli download --chip nrf52840 --format hex s140_nrf52_7.3.0_softdevice.hex
7. in device nrf-softdevice/examples run: cargo run --bin ble_bas_peripheral
8. Maybe edit RAM in Cargo.toml according to https://github.com/embassy-rs/nrf-softdevice section "Configuring a SoftDevice"