# docker build --output=output --target=binaries .
FROM ubuntu:22.04 as builder

RUN apt-get update
RUN apt-get install -y \
    build-essential \
    curl

RUN apt-get update

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt-get -y install build-essential python3-pip curl
RUN apt-get -y install git python3 ninja-build pkg-config

# tested with this commit specifically - hopefully works with later / current as well
RUN git clone --recursive https://github.com/tock/libtock-rs && cd libtock-rs && git checkout 482be6f0c6c33df50ca774a807dea84164aecd82

# custom qemu build
RUN apt-get -y install libglib2.0-dev libfdt-dev libpixman-1-dev zlib1g-dev libgtk-3-dev

RUN cd libtock-rs && make setup

# copy our examples in
COPY examples /libtock-rs/examples

# build examples
RUN cd libtock-rs && make nrf52840 EXAMPLE=ble_advertising
RUN cd libtock-rs && make nrf52840 EXAMPLE=ble_scanning

# build tock
RUN cd libtock-rs/tock/boards/nordic/nrf52840dk && make all

# copy these files out at the end
RUN mkdir output

# Useful for flashing to device
RUN cp libtock-rs/tock/target/thumbv7em-none-eabi/release/nrf52840dk.bin output/nrf52840dk.bin
RUN cp libtock-rs/target/thumbv7em-none-eabi/release/examples/ble_advertising.tab output/ble_advertising.tab
RUN cp libtock-rs/target/thumbv7em-none-eabi/release/examples/ble_scanning.tab output/ble_scanning.tab

# useful for renode 
RUN apt-get -y install gcc-arm-none-eabi

RUN arm-none-eabi-objcopy --update-section .apps=libtock-rs/target/thumbv7em-none-eabi/release/examples/ble_advertising.tbf libtock-rs/tock/target/thumbv7em-none-eabi/release/nrf52840dk.elf output/ble_advertising.elf
RUN arm-none-eabi-objcopy --update-section .apps=libtock-rs/target/thumbv7em-none-eabi/release/examples/ble_scanning.tbf libtock-rs/tock/target/thumbv7em-none-eabi/release/nrf52840dk.elf output/ble_passive_scanning.elf

FROM scratch AS binaries
COPY --from=builder /output/nrf52840dk.bin /
COPY --from=builder /output/ble_advertising.tab /
COPY --from=builder /output/ble_scanning.tab /
COPY --from=builder /output/ble_advertising.elf /
COPY --from=builder /output/ble_passive_scanning.elf /