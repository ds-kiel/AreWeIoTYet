# Introduction
## Motivation
Rust is interesting, promising language with a lot of research and active development with an ongoing demand.  Additionally, it is a promising alternative to C, which would be great to make IoT development easier accessible. We want to be able to try out and efficiently automized test/serialize our implementations, thus we give instructions on how to test an implementation in Renode, an emulator, before flashing it to the real boards. BLE is common in IoT, everyday devices, multiple development boards and frameworks. It is low-energy, efficient, designed for smart devices, seamless connectivity and is widely spreaded.

## IoT vs Embedded
[Embedded](https://www.rust-lang.org/what/embedded) and [RTOS](https://arewertosyet.com) is already covered in Rust. As some of the OSs were very promising, we pushed for IoT by also including over-the-air communication into our research. We especially focused on Bluetooth Low Energy (BLE) as it was the most interesting radio to us. So we focused on the communication and its automized testing.

## Why Rust
To build memory-safety, reliable, stable software. Has no race-conditions but low-level programming if needed (good for IoT context)

## Why this setup
Why nrf52840dk: All OSs we used have specific support for nrf52840dk which is relevant as we don't only need the support for the board but also for the peripherals.

Why Renode: Scalable, simulation framework with IoT. Vastly improve testing capabilities. Open Source, full determinism, continous integration. Supports Nrf52840dk boards Transparent and robust debugging.

## Expectations
We expect Renode to behave the same as the boards on the same code. BLE communication should be doable in Rust language. Not neccessary to be able to write C code. Implementations should run stable. Maintained Rust support for the OSs (future proof). BLE stack should be future rich.