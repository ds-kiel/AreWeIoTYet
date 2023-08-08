# Examples

RIOT OS provides a few [examples](https://github.com/RIOT-OS/RIOT/tree/master/examples) for Bluetooth Low Energy using both their Nimble and Skald drivers, but they are written in C.
There are also a few [Rust examples](https://github.com/RIOT-OS/RIOT/tree/master/examples/rust-hello-world).

RIOT OS [system for using Rust](https://doc.riot-os.org/using-rust.html) is pretty decent but the [Wrapper Library](https://github.com/RIOT-OS/rust-riot-wrappers) is as of the writing of this still lacking in quite a lot of aspects, especially when it comes to BLE.
This leads to us having to write wrapper code for the C interface our self by mirroring C structures and a lot of `unsafe` sections.

## BLE Advertising

The [Rust BLE Advertising Example](TODO: LINK HERE) uses Nimble to send advertisements.

This is how it looks like in renode:

![](advertise.png)

And this is how the advertisement packets that are send are structured:

![](packet.png)

## BLE Scanning

The [Rust BLE Scanning Example](TODO: LINK HERE) registers a `scan` command and when it is entered either with or without a timeout argument it waits that long and prints out the received advertisements.

When running the [`riotos_demo`](build.html#riotos_demo) this is what the scanner receives:

![](receive.png)