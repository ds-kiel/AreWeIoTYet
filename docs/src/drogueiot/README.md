# Drogue IoT

## [😄] Bluetooth Status

Drogue builds on embassy which is using  Nordic Semiconductor's [SoftDevice](https://infocenter.nordicsemi.com/topic/struct_nrf52/struct/nrf52_softdevices.html). The SoftDevice is Bluetooth 5.1 qualified, and very well tested probably all needed features will be available.


## [😄] Rust Status

Drogue and Embassy are written in Rust, and all the examples are also written in Rust.
There is exactly one [BLE example](https://book.drogue.io/drogue-device/dev/examples.html) for the nRF 52840 DK board but there are also three further examples for other boards.


## [☹️] Renode Status

As Drogue builds on top of Embassy, this status is as bad as it is for Embassy.  

Embassy itself does boot in Renode but [getting output and debug information](renode.html#general-problems) as Embassy intends is not supported.
It is also possible to boot Embassy in Renode [with the SoftDevice enabled](renode.html#booting-with-the-softdevice-solved).

We were not successful in getting Embassy on Renode to stably work with the Bluetooth features of the Softdevice.

## Prerequisites
For Ubuntu 22.04 make sure that the following are installed:
- pkg-config
- libudev-dev
- probe-run
- probe-rs-cli  
- bluez
- bluez-meshd  