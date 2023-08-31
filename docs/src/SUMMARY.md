# Summary
There were already people who evaluated embedded and Real-Time OS for Rust. On going we made a survey on BLE communication using Rust. So we answer the question: Is it possible to write maintainable BLE devices in Rust and to test them? Therefore we used the nrf52840dk boards as they are supported by many of the embedded frameworks and tried BLE on them. Additionally, we tried Renode as an simulator for testing purposes without the need to directly flash the implementation on a board. Our results are that there is a lot to do. So. Are we IoT Yet? No, but if only using boards and missing out the testing on a simulatior embassy is a good way to do it. If it is neccessary to use the simulator for testing then RiotOS is a well-tested major RTOS that is used in industry and on universities. But it is written in C and also for writing Rust it is currently neccessay to be skilled in Rust and C as well.



[Summary](README.md)

- [Introduction](introduction.md)
    - [Bluetooth Stack Comparison](introduction/bluetooth.md)
- [Setup](setup.md)
    - [Hardware](setup/hardware.md)
    - [Renode](setup/renode.md)
- [Operation Systems](os.md)
    - [RiotOS](riotos/README.md)
        - [Examples](riotos/examples.md)
        - [Built & Run Instructions](riotos/build.md) 
    - [TockOS](tockos/README.md)
        - [Examples](tockos/examples.md)
        - [Built & Run Instructions](tockos/build.md) 
    - [Embassy](embassy/README.md)
        - [Examples](embassy/examples.md)
        - [Built& Run Instruction for Board](embassy/board.md) 
        - [Remarks regarding Renode](embassy/renode.md)
    - [Drogue IoT](drogueiot/README.md)
        - [Examples](drogueiot/examples.md)
    - [ZepyhrOS](zephyros/README.md)
- [Conclusion](conclusion.md)