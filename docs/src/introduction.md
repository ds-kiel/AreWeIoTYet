# Introduction

## Motivation
There is an ongoing demand from developers to use Rust as an alternative to C as it is an interesting and promising language with a lot of active development and research. As a promising alternative to C, it would also make embedded and IoT development easily accessible. Rust is a programming language to build memory-safe, reliable, and stable software without race-conditions which is great for IoT applications. Thus, we want to check the state of IoT development using Rust. So, we answer the question "**Are We IoT Yet?**". Therefore, we used two subquestions to be answered on our way:
1. Is it possible to write maintainable IoT devices in Rust?
2. Is it possible to deeply test the implementations safely in a simulator?

## IoT vs. Embedded
Overall, the big difference between embedded devices and IoT devices is the communication done with IoT devices. IoT devices are connected which is typically done via some wireless link. As Bluetooth Low Energy (BLE) is common these days not only in IoT devices but also in everyday devices, multiple development boards, and frameworks, we focused on wireless communication using BLE as it is widely spread. Additionally, BLE is designed for smart devices, so it is efficient and low-energy-consuming.  

The state of [Embedded](https://www.rust-lang.org/what/embedded) and [RTOS](https://arewertosyet.com) in Rust is already covered. Although RTOS is not that well supported in the tested frameworks, we pushed for IoT development as at least embedded development in Rust has [official support](https://github.com/rust-embedded/wg) and some of the operating systems of the [embedded Rust list](https://github.com/rust-embedded/awesome-embedded-rust) looked promising. 

<!-- Removed ## Why Rust, as it is already answered in the first ## Motivation block -->

## Setup
To be able to answer the subquestion on the safe test environment, we chose Renode as a simulator to run our implementations before flashing them to the real hardware. Renode is a scalable simulation framework, especially for IoT devices. It is Open Source, fully deterministic, and stands out due to its continuous integration. Additionally, Renode can be used on most operating systems.

For the hardware side, we chose Nordic Semiconductors nRF52840dk board as it is quite common and well supported for multiple frameworks. It is relevant that we used this exact board as we didn't just go for our chip being supported but also for the peripherals of the board. Additionally, Renode supports simulating these boards which is great for transparent and robust debugging before flashing on the boards. It is also useful if one does not have a board (yet) and already wants to start implementing and debugging.

## Expectations
We expect Renode to behave the same as the boards when given the same code. Additionally, we expect BLE communication implementable using Rust without writing C code as we want new developers to only focus on programming Rust. The implementations should run stable <!-- TODO: Sepcify stable! -->. Additionally, we would expect maintained Rust support for the operating systems as otherwise they most likely will not have any future value to the industry. Furthermore, we like the BLE stack of each operating system to be feature-rich, meaning more than a minimal implementation is required as most applications will need connectivity of the devices.