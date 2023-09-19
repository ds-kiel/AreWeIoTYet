# Tock

Tock is an embedded operating system written in Rust.
It is designed to be able to run multiple concurrent, mutually distrustful applications on embedded platforms.
For us most notably it explicitly supports the nRF52840 DK board and minimal BLE.

## [😐] Bluetooth Status

As of the writing of this, the [tock Bluetooth stack](https://github.com/tock/tock/blob/master/capsules/extra/src/ble_advertising_driver.rs) only supports advertising and passive scanning.

## [🙂] Rust Status

Tock is written in Rust which makes it a very good candidate for developing code in Rust.
But for user application code the [libtock C library](https://github.com/tock/libtock-c/tree/master/examples) contains way more detailed examples than the [libtock Rust library](https://github.com/tock/libtock-rs/tree/master/examples).

For BLE specifically there are only [`ble_advertising`](https://github.com/tock/libtock-c/tree/master/examples/ble_advertising) and [`ble_passive_scanning`](https://github.com/tock/libtock-c/tree/master/examples/ble_passive_scanning) examples written in C. 
We additionally provide Rust [`ble_advertising`](examples.html#ble-advertising) and [`ble_scanning`](examples.html#ble-scanning) examples, but notable in them is that to use the BLE features we had to directly use Tock's Syscall Interface as there are no wrappers provided for doing so.


## [😄] Renode Status

Tock works in Renode [with an adjusted CPU configuration](build.html#notes-on-renode) that accounts for some missing registers which otherwise cause a bootloop.

Provided are [Dockerfiles for building](build.html#build-with-docker) the examples and directly [running them in renode](build.html#run-on-renode).