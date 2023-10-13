# Build & Run Instructions


For the Rust components we first need [`rustup`](https://rustup.rs/):

- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`


For building the `c2rust` bindings we need a few more dependencies, mainly a `LLVM` toolchain:

```
# ubuntu:22.04

apt-get install cmake pkg-config

apt-get install \
        llvm-14 \
        clang-14 \
        clang-tools-14 \
        lld-14 \
        llvm \
        clang \
        clang-tools

apt-get install \
        libclang-dev \
        libssl-dev \
        llvm-dev \
        libclang-14-dev
        
cargo install c2rust --git https://github.com/immunant/c2rust
```

[Building `RIOT`](https://doc.riot-os.org/index.html#the-quickest-start) itself only requires a few build tools

- `apt-get install gcc-arm-none-eabi make gcc-multilib libstdc++-arm-none-eabi-newlib openocd gdb-multiarch doxygen wget unzip python3-serial` (ubuntu:22.04)

We then configure `rustup` to use the right toolchain:

- `rustup target add thumbv7em-none-eabihf --toolchain stable`

clone `RIOT`:

- `git clone https://github.com/RIOT-OS/RIOT`

copy our examples into the `examples` folder and build them:

- `cd RIOT && make BOARD=nrf52840dk -C examples/rust-nimble_advertiser`
- `cd RIOT && make BOARD=nrf52840dk -C examples/rust-nimble_scanner`

The build artifacts we care about in the end are:

- `RIOT/examples/rust-nimble_advertiser/bin/nrf52840dk/nimble_advertiser.elf` - The build advertiser
- `RIOT/examples/rust-nimble_advertiser/bin/nrf52840dk/nimble_scanner.elf` - The build scanner


## Build with Docker

A `Dockerfile` is additionally provided that when run will build all these artifacts [here](https://github.com/ds-kiel/AreWeIoTYet/tree/main/riot/output).

By running `docker build --output=output --target=binaries .` The docker image will be built and the output artifacts will be extracted and put into a folder called `output`.

## Run on Device

The [recommended way](https://doc.riot-os.org/flashing.html) of flashing and interacting with the device is using `RIOT`'s build system:

`PROGRAMMER=jlink make BOARD=nrf52840dk -C examples/<example> flash `
`PROGRAMMER=jlink make BOARD=nrf52840dk -C examples/<example> term`

This will directly rebuild and flash the examples to the device.

## Run on Renode
The build ELF files can directly be loaded into `renode`.

The examples provide two `renode` scenarios to play around with: `riot_advertise` and `riot_demo`.

### `riot_advertise`

Run with:

- `renode -e "include @riot_advertise.resc;start"`

This starts the advertising example and Wireshark to capture the Bluetooth Low Energy Traffic:

![](advertise.png)
![](packet.png)

### `riot_demo`

Run with:

- `renode -e "include @riot_demo.resc;start"`

This starts both the advertising and scanning examples.


### Notes on renode:

The wiki of RIOT has a [small section](https://doc.riot-os.org/emulators.html) on how it already supports Renode as an emulator, but when trying to emulate device-specific functionality like BLE we still have a problem for the NRF boards.
Specifically the way RIOT implemented UART for the nRF52840 by relying on the `Shortcut` functionality that is not implemented (as of yet) in Renode's nRF52840 UART driver.
This means we need to load a patched [`NRF52840_UART_MODIFIED.cs`](https://github.com/ds-kiel/AreWeIoTYet/blob/main/riot/NRF52840_UART_MODIFIED.cs) driver and use it instead. 