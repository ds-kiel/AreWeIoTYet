# Summary
We got inspired by the work of [Are We RTOS Yet?](https://arewertosyet.com/) who did a small survery on the state of RTOS and Rust. As tehy see a bright future for RTOS and Rust we thought about the question **Are We IoT Yet?**. We specialized on the part of wireless communication of the question as tiny embedded devices are typically connected wireless via radio and the status of [embedded devices in Rust](https://github.com/rust-embedded/wg) is quite good and has official support. Our focus is on Bluetooth Low Energy (BLE) as it is one of the most important radios to be used when connecting sensors.

To answer the question **Are We IoT Yet?** we defined two subquestions to be answered for each approached OS-BLE-stack-combination:  
1. Is it possible to write maintainable BLE devices in Rust?
2. Is it possible to deeply test the implementations safely in a simulator?

For simulation we used [Renode](https://renode.io/) while using the nRF52840dk board as hardware to test and compare the results on.  

In this book we provide a comparison on five frameworks we tested in terms of BLE communication using our setup. Additionally, we give instructions and examples for each of the frameworks.

Our results are that the perfect framework to develop IoT devices with purely Rust does not exists yet. If it is feasible to miss out on a simulation and safe testing environment then [Embassy](embassy/README.md) is a good ways to develop IoT devices. If it is neccessary to simulate implementations before flashing them to real hardware then [RIOT](riot/README.md) is a well-tested major RTOS that is used in industry and on universities. But for writing Rust it is currentyl neccessary to be skilled and C as well.  