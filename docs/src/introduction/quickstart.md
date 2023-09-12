# Quickstart Guide

We provide examples, Dockerfiles, and Renode scenario files for [tock](../tock) and [RIOT](../riot) in the [repository](https://github.com/Pusty/AreWeIoTYet).
These files act both as a demonstration of how Rust development for these frameworks looks like and as an entry-point to trying it out.

## Setup

To build the examples only [Docker](https://docs.docker.com/engine/install/) needs to be installed.

- Clone the repository: `git clone https://github.com/Pusty/AreWeIoTYet`
- Build tock: `cd tock;docker build --output=output --target=binaries .;cd ..`
- Build RIOT: `cd riot;docker build --output=output --target=binaries .;cd ..`

To run the scenarios [Renode](introduction/renode) and [Wireshark](https://www.wireshark.org/download.html) need to be installed.

- Run the RIOT advertiser example: `cd riot;renode -e "include @riot_advertise.resc;start"`
- Run a RIOT demonstration of advertiser and scanner: `cd riot;renode -e "include @riot_demo.resc;start"`

- Run the tock advertiser example: `cd tock;renode -e "include @tock_advertise.resc;start"`
- Run a tock demonstration of advertiser and scanner: `cd tock;renode -e "include @tock_demo.resc;start"`