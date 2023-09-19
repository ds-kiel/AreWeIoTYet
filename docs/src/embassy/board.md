# Build & Run Instruction for Board

### Prerequistits
For Ubuntu 22.04 make sure that the following are installed:
1. pkg-config
2. libudev-dev
3. probe-run
4. probe-rs-cli  
  
If they are not yet installed, do:
 - `apt-get install libudev-dev probe-run probe-rs-cli` (Ubuntu 22.04)

## Embassy without Softdevice

### Install Embassy
This setup is not able to use BLE and only includes the operating system. The embassy team provides build and run instructions in [their repository](https://github.com/embassy-rs/embassy).

 - Clone Embassy:  
 `git clone https://github.com/embassy-rs/embassy.git`
 - Update Submodules:  
 `git fetch`  

Run Embassy example
 - Change directory to the cloned embassy directory
 - Then change to the examples folder of the board:  
 `cd examples/nrf52840`
 - Connect the board 
 - Clear the board:  
`probe-rs-cli erase --chip nrf52840`
 - Use `cargo run` to build and run the example on the board. For the blinky example, do:  
 `cargo run --bin blinky --release`  

Now the example should perform as expected. For the blinky example this is a blinking LED 1. Other examples are described in the [next subchapter](examples.md).
  
## Combining Embassy with Softdevice
The instructions are described in the [embassy-rs/nrf-softdevice repoository](https://github.com/embassy-rs/nrf-softdevice). Below you can find a similar instruction.

Get the SoftDevice and the embassy version that is capable of using it:
 - Clone the nrf-softdevice repository:  
 - Download a fitting SoftDevice from Nodics Webpage. I used S140, version 7.3.0
 - Unzip the SoftDevice archive into the examples folder of the nrf-softdevice repository  
 
 After that prepare rust and the board:
 - Run:   
 `rustup update`  
 - Clear the board:  
 `probe-rs-cli erase --chip nrf52840` 

Now flash the SoftDevice to the board:
 - Flash the SoftDevice to the board:   
 `probe-rs-cli download --chip nrf52840 --format hex s140_nrf52_7.3.0_softdevice.hex` 
  - If necessary edit the RAM in Cargo.toml according to [https://github.com/embassy-rs/nrf-softdevice](https://github.com/embassy-rs/nrf-softdevice), section "Configuring a SoftDevice"  

Run an embassy-softdevice example:  
 - To run the embassy BLE BatteryService Peripheral, do:  
 `cd examples && cargo run --bin ble_bas_peripheral`