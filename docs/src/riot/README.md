# RIOT

RIOT is a real-time multi-threading operating system written in C.
It has support for a large variety of devices, peripherals, and protocols.
For us most notably it explicitly supports the `nrf52840dk` board and BLE through a port of the [Apache Mynewt NimBLE](https://mynewt.apache.org/latest/network/) stack.

## [😄] Bluetooth Status

NimBLE is fully compliant with the Bluetooth 5.4 specifications with support for Bluetooth Mesh.
Also notably there exists [official certification](https://cwiki.apache.org/confluence/display/MYNEWT/RN-NimBLE-1.1.0) for the version 1.1.0 of NimBLE that is Bluetooth 5.0 compliant.

## [🙂] Rust Status

RIOT has official Rust support and [proper instructions on how to use it](https://doc.riot-os.org/using-rust.html).
Generally, it works by running `C2Rust` and generating Rust bindings for the C code automatically.
In addition, there are provided [wrappers](https://github.com/RIOT-OS/rust-riot-wrappers) that provide more proper methods of interacting with some components.
At the time of writing, there are no provided wrappers for using the NimBLE stack, so BLE interaction is done in `unsafe {}` blocks using the Rust to C bindings.

By default, there is only a `rust-hello-world` and `rust-gcoap` example. We additionally provide a [`rust-ble-advertising`](examples.html#ble-advertising) and [`rust-ble-scanning`](examples.html#ble-scanning) example.

## [😄] Renode Status

RIOT works in Renode [with a modified UART driver](build.html#notes-on-renode) that accounts for the way RIOT implemented their serial output.
Thanks to the architecture of Renode, this modified UART driver can be provided externally.

Provided are [Dockerfiles for building](build.html#build-with-docker) the examples and directly [running them in renode](build.html#run-on-renode).