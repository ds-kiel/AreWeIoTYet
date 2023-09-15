# Embassy



## [😄] Bluetooth Status

As Embassy is using Nordic Semiconductor's [SoftDevice](https://infocenter.nordicsemi.com/topic/struct_nrf52/struct/nrf52_softdevices.html) which is Bluetooth 5.1 qualified, and very well tested probably all needed features will be available.


## [😄] Rust Status

Embassy is written in Rust, and all the examples are also written in Rust.
A wide selection of [BLE examples](https://github.com/embassy-rs/nrf-softdevice/tree/master/examples/src/bin) meant to be used with the SoftDevice is also directly available.


## [☹️] Renode Status

Embassy itself does boot in Renode but [getting output and debug information](renode.html#general-problems) as Embassy intends is not supported.
It is also possible to boot Embassy in Renode [with the SoftDevice enabled](renode.html#booting-with-the-softdevice-solved).

We were not successful in getting Embassy on Renode to work with the Bluetooth features of the Softdevice.