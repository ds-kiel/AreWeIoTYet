# Embassy



## [😄] Bluetooth Status

As Embassy is using Nordic Semiconductor's [SoftDevice](https://infocenter.nordicsemi.com/topic/struct_nrf52/struct/nrf52_softdevices.html) which is Bluetooth 5.1 qualified, and very well tested probably all needed features will be available.


## [😄] Rust Status

Embassy is written in Rust, and all the examples are also written in Rust.
A wide selection of [BLE examples](https://github.com/embassy-rs/nrf-softdevice/tree/master/examples/src/bin) meant to be used with the SoftDevice are also directly available.


## [☹️] Renode Status

Embassy itself does boot in Renode but [getting output and debug information](renode.html#general-problems) as Embassy intends is not supported.
It is also possible to boot Embassy in Renode [with the SoftDevice enabled](renode.html#booting-with-the-softdevice-solved).

We were not successful in getting Embassy on Renode to stably work with the Bluetooth features of the Softdevice.



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