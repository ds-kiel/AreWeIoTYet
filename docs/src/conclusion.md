# Conclusion
No, we are not IoT yet. But there is a way to get there.
 - ToDo List for improvable things: (Renode support for embassy, Good Rust wrapper for C for RIOT, maintain Rust for Zephyr, ...)

## Goals
Inspired by [Are We RTOS Yet?](https://arewertosyet.com) we did our own survey regarding IoT development. Therefore, we started with some of the [embedded tools](https://github.com/rust-embedded/awesome-embedded-rust#tools) like Embassy, Tock and RIOT. We added Drogue as extension to Embassy and ZephyrOS as common operating system in the IoT domain. We went from embedded development and RTOS to IoT development by including wireless communication, namely BLE. Additionally, we wanted to simulate our implementations for testing and debugging purposes in a simulator before flashing them on our boards. So we used Renode as it is a simulator for exactly those purposes which is able to simulate the nRF52830dk board from Nordic Semiconductor which is the one we used.   

## Results
Bringing all these parts together, we found that none of the researched frameworks fullfilles all our requirements. While Embassy is fully written in Rust, it lacks its own BLE stack and uses the SoftDevice from Noric Semiconductor which is written in C. This way they enable the full usage of the boards capabilities but made it impossible for us to test our implementations on Renode. The same holds for Drogue as it basically is just an extension to Embassy. Tock worked with Renode with only litte adjustments. Sadly, it has a minimal BLE stack which makes it useless for many purposes. ZephyrOS works good with Renode. However, there is no official support for Rust in Zephyr and the combination of Rust and Zephyr misses any support for Bluetooth features. RIOT does great with Bluetooth and with Renode. Additionally, it also has official support for Rust. But currently developing IoT applications in Rust on RIOT still needs C knowledge.   

Summarizing one can say that Embassy and RIOT are the best ways to go but both with their own restrictions.

## Future Work
For reaching a good status on developing IoT applications using Rust and simulate the code for testing and debugging purposes we provide a list of ToDo's below:  
 - Add Renode support for Embassy (and Drogue IoT)  
 - Implement good Rust wrapper for C for RIOT  
 - Maintain Rust for Zephyr
 - Build a full Bluetooth stack for Tock   