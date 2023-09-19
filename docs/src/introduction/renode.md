# Renode

[Renode](https://renode.io/) is an open source software development framework that lets you debug and test multi-node device systems.
Through it, we can deterministically simulate scenarios and debug them before or in addition to testing on real devices.
This generally makes it faster and easier to develop and verify behavior.
Compared to directly emulating the processor and testing using that (e.g. directly using [qemu](https://www.qemu.org/)) Renode has abstraction for specific board features and peripherals and has a plugin system to add components or modify their behavior.

For general information, the [Renode Documentation](https://renode.readthedocs.io/en/latest/) is very helpful, but as a starting point a bit confusing.
For reference on how peripherals are implemented the [Source Code](https://github.com/renode/renode-infrastructure/tree/master/src/Emulator/Peripherals/Peripherals) provides a good reference.

## Installation

To install Renode you just need to download the appropriate [release](https://github.com/renode/renode/releases/latest) for your operation system and install it.
For Windows this is the `.msi` installer, for macOS the `.dmg` disk image, for Debian/Ubuntu this is the `.deb` software package.

After installation, you might want to make sure that the installed program's `bin` folder is part of your PATH variable so you can access the `Renode` binary everywhere.

Optionally install [Wireshark](https://www.wireshark.org/download.html) for logging Bluetooth Low Energy Traffic.

## Installed Platforms

Inside your `Renode/platforms/cpus` folder the configurations for different CPUs are listed and the `Renode/platforms/boards` folder contains a list of all supported boards.
These files act as a good reference when you need to adjust your board configuration to add or modify peripherals.
The `Renode/scripts` folder contains a collection of different simulations to try out and see how they are implemented.

## Running a Simulation

For our specific use case where we already decided on the nRF52840 DK board and know we want to either run single-node or multi-node Bluetooth Low Energy simulations, building our scenario is fairly simple:

Here is a fully commented version of the [tock BLE advertise example](https://github.com/Pusty/AreWeIoTYet/blob/main/tock/tock_advertise.resc), which is oriented on the official [BLE Tutorial](https://renode.readthedocs.io/en/latest/tutorials/ble-simulation.html#bluetooth-low-energy-simulation-in-renode):

```
# The system bus is the core peripheral that is always defined
using sysbus

# Setup a wireless medium
# https://renode.readthedocs.io/en/latest/networking/wireless.html
emulation CreateBLEMedium "wireless"

# Create another node
mach create

# Load the CPU/Board description from the "nrf52840-tock.repl" file into the current node
machine LoadPlatformDescription @nrf52840-tock.repl

# Connect the current node to the BLE network
connector Connect sysbus.radio wireless

# Set the variable $bin if not set already to the path of "output/ble_advertising.elf"
$bin?=@output/ble_advertising.elf

# Create another window that outputs and interacts with uart0 (defined in the Platform Description)
showAnalyzer uart0

# Control what is logged, in this case NOISY/everything
# https://renode.readthedocs.io/en/latest/basic/logger.html
logLevel -1

# Define some time scale, this is not explained in detail but is necessary 
emulation SetGlobalQuantum "0.00001"

# Optionally logging of BLE Traffic with Wireshark
# https://renode.readthedocs.io/en/latest/networking/wireshark.html
emulation LogBLETraffic

# This macro method seems to be the preferred method to script the reset
# In this case it is just loading the binary ELF file on the one node
macro reset
"""
    sysbus LoadELF $bin
"""
runMacro $reset
```

Notable here is that for setting up BLE we need to create the medium, connect our node to it, and set up some emulation time scale which is not really explained anywhere in detail.

Interesting is the `emulation LogBLETraffic` feature which if [Wireshark](https://www.wireshark.org/) is installed will open it and log the BLE traffic.


To run this simulation we now only need to start Renode, then in the Renode console load the script with the command `include @tock_advertise.resc` and start the simulation with the command `start`.
This can be done in one command from the command line as well as `renode -e "include @tock_advertise.resc;start"`

## Running multi-node simulations

Here is a fully commented version of the [tock BLE scanning example](https://github.com/Pusty/AreWeIoTYet/blob/main/tock/tockos_demo.resc):

```
# The system bus is the core peripheral that is always defined
using sysbus

# Set variables for the binary paths for the advertiser and scanner 
$adv_bin?=@output/ble_advertising.elf
$scan_bin?=@output/ble_passive_scanning.elf

# Setup a wireless medium
# https://renode.readthedocs.io/en/latest/networking/wireless.html
emulation CreateBLEMedium "wireless"


# Create the Advertiser Node under the name "adv"
mach create "adv"

# Load the CPU/Board description from the "nrf52840-tock.repl" file into the "adv" node
machine LoadPlatformDescription @nrf52840-tock.repl

# Connect the "adv" node to the BLE network
connector Connect sysbus.radio wireless

# Spawn a window for interacting with the uart0 interface of the "adv" node
showAnalyzer uart0


# Create the Scanner Node under the name "scan"
mach create "scan"

# Load the CPU/Board description from the "nrf52840-tock.repl" file into the "scan" node
machine LoadPlatformDescription @nrf52840-tock.repl

# Connect the "scan" node to the BLE network
connector Connect sysbus.radio wireless

# Spawn a window for interacting with the uart0 interface of the "scan" node
showAnalyzer uart0 

# Define some time scale, this is not explained in detail but is necessary 
emulation SetGlobalQuantum "0.00001"

# Optionally logging of BLE Traffic with Wireshark
# https://renode.readthedocs.io/en/latest/networking/wireshark.html
emulation LogBLETraffic

# On reset:
# Load the advertiser binary into the "adv" node
# Load the scanner binary into the "scan" node
macro reset
"""
    mach set "adv"
    sysbus LoadELF $adv_bin

    mach set "scan"
    sysbus LoadELF $scan_bin 
"""
runMacro $reset
```

Again to run this scenario we can include the scenario and start, or just run `renode -e "include @tock_demo.resc;start"`.

To add more nodes we need to make that we create the machine, load a description, and connect it to the BLE network:

```
mach create "name"
machine LoadPlatformDescription @nrf52840-tock.repl
connector Connect sysbus.radio wireless

# If we want UART console input/output
showAnalyzer uart0 
```

In the reset macro we need to add the commands to load the appropriate programs into them:

```
    mach set "name"
    sysbus LoadELF @path/to/binary
```

## Extendability

For both [tock](../tock) and [RIOT](../riot), we ran into issues where the stock CPU configuration for the nRF52840 was not enough to properly simulate programs.
For tock, this meant [adding additional registers](../tock/build.html#notes-on-renode) which is possible by just modifying the platform description and adding them.
See the [tock scenarios](https://github.com/Pusty/AreWeIoTYet/blob/main/tock/tock_demo.resc) as an example of this.

For RIOT, the problem was the underlying [UART implementation](../riot/build.html#notes-on-renode) for the nRF52840, but this also is not a problem because Renode lets you load in your own peripherals as C# files.
Then at the start of your scenario file you load these custom C# peripherals in just like a scenario script with `include @NRF52840_UART_MODIFIED.cs` which you can then reference in e.g. your platform description file.
See the [RIOT scenarios](https://github.com/Pusty/AreWeIoTYet/blob/main/riot/riot_advertise.resc) as an example of this.

