# docker build --output=output --target=binaries .
FROM ubuntu:22.04 as builder

RUN apt-get update
RUN apt-get install -y \
    build-essential \
    curl

RUN apt-get update

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt-get -y install git python3 python3-pip python3-serial

RUN apt-get -y --no-install-recommends install cmake pkg-config
        
RUN apt-get -y --no-install-recommends install \
        llvm-14 \
        clang-14 \
        clang-tools-14 \
        lld-14 \
        llvm \
        clang \
        clang-tools

RUN apt-get -y --no-install-recommends install \
        libclang-dev \
        libssl-dev \
        llvm-dev \
        libclang-14-dev
        
RUN cargo install c2rust --git https://github.com/immunant/c2rust

RUN apt-get -y --no-install-recommends install gcc-arm-none-eabi make gcc-multilib libstdc++-arm-none-eabi-newlib openocd gdb-multiarch doxygen wget unzip python3-serial

RUN rustup target add thumbv7em-none-eabihf --toolchain stable

# tested with this commit specifically - hopefully works with later / current as well
RUN git clone https://github.com/RIOT-OS/RIOT && cd RIOT && git checkout e1ea18fea25a039fd9fc3a9b74697da0c029718a

# copy our examples in
COPY examples /RIOT/examples

RUN cd RIOT && make BOARD=nrf52840dk -C examples/rust-nimble_advertiser
RUN cd RIOT && make BOARD=nrf52840dk -C examples/rust-nimble_scanner

RUN mkdir output

RUN cp RIOT/examples/rust-nimble_advertiser/bin/nrf52840dk/nimble_advertiser.elf /output/rust-nimble_advertiser.elf
RUN cp RIOT/examples/rust-nimble_advertiser/bin/nrf52840dk/nimble_advertiser.bin /output/rust-nimble_advertiser.bin

RUN cp RIOT/examples/rust-nimble_scanner/bin/nrf52840dk/nimble_scanner.elf /output/rust-nimble_scanner.elf
RUN cp RIOT/examples/rust-nimble_scanner/bin/nrf52840dk/nimble_scanner.bin /output/rust-nimble_scanner.bin

FROM scratch AS binaries
COPY --from=builder /output/rust-nimble_advertiser.bin /
COPY --from=builder /output/rust-nimble_advertiser.elf /
COPY --from=builder /output/rust-nimble_scanner.bin /
COPY --from=builder /output/rust-nimble_scanner.elf /