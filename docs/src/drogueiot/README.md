# Drogue IoT

## [😄] Bluetooth Status

Drogue builds on embassy which is using  Nordic Semiconductor's [SoftDevice](https://infocenter.nordicsemi.com/topic/struct_nrf52/struct/nrf52_softdevices.html). The SoftDevice is Bluetooth 5.1 qualified, and very well tested probably all needed features will be available.


## [😄] Rust Status

Drogue and Embassy are written in Rust, and all the examples are also written in Rust.
There is exactly one [BLE example](https://book.drogue.io/drogue-device/dev/examples.html) for the nRF 52840 DK board but there are also three further examples for other boards.


## [☹️] Renode Status

As Drogue builds on top of Embassy, this status is as bad as it is for Embassy.  

Embassy itself does boot in Renode but [getting output and debug information](renode.html#general-problems) as Embassy intends is not supported.
It is also possible to boot Embassy in Renode [with the SoftDevice enabled](renode.html#booting-with-the-softdevice-solved).

We were not successful in getting Embassy on Renode to stably work with the Bluetooth features of the Softdevice.

## Setup Drogue-IoT / Drogue-Device

It is necessary to open multiple terminal windows to setup and run the example.

## Prerequisites
For Ubuntu 22.04 make sure that the following are installed:
- pkg-config
- libudev-dev
- probe-run
- probe-rs-cli  
- bluez
- bluez-meshd  

## Setup Example
Clone the drogue github repository and change to its directory:  
```
git clone https://github.com/drogue-iot/drogue-device.git  
cd drogue-device
```

To later be able to stop the bluetooth mesh process, do now:
```
sudo service bluetooth stop  
sudo /usr/lib/bluetooth/bluetooth-meshd -nd --debug
```

Next, we need a new terminal window as this one will stay busy. There we start the mesh network:
```
mesh-cfgclient
```
In case an error occurs about not finding the ```config_db.json```, run:
```
[mesh-cfgclient]# create
```
Now we created the mesh network to which we want to add the node running the drogue example.

In another new terminal window, change directory to the bluetooth mesh example:  
```
cd <drogue-device-git>/examples/nrf52/nrf52840-dk/bt-mesh
```

To run the example without seeing the debug messages, do:
```
cargo run --release
```
The output should be: ```(HOST) INFO success!```

### USB Permissions
This might run into an error:
```
USB error while opening USB device: Access denied (insufficient permissions)
```
To read the USB permissions on Ubuntu 22.04 for the board, do:
```
lsusb  
```
This will output an id for the connected board which we will use in the next step:  
```
lsusb -d <id for the board>
```
You will get the bus number and the device number here. To now read the permissions for the board using the acquired numbers, do:
```
ls -l /dev/bus/usb/<bus-nr>/<device-nr>
```
The output should look like: ```crw-rw-r-- 1 root root 189, 8 Jul 18 06:48```  

Now, to modify the USB permissions open the file ```/etc/udev/rules.d/50-oxidize-global.rules``` and add the board:  
```
# udev rules to allow access to USB devices as a non-root user

# nRF52840 Dongle in bootloader mode
ATTRS{idVendor}=="1915", ATTRS{idProduct}=="521f", TAG+="uaccess"

# nRF52840 Dongle applications
ATTRS{idVendor}=="2020", TAG+="uaccess"

# nRF52840 Development Kit
ATTRS{idVendor}=="1366", ENV{ID_MM_DEVICE_IGNORE}="1", TAG+="uaccess"
```

Then reload the permission rules:  
```
sudo udevadm control --reload-rules
```  
It might be interesting to read the permissions again at this time. This is then done by repeating the first steps. The output should be changed from:
``` crw-rw-w-- 1 ``` to ```crw-rw-r--+ 1```.  
Now, retry to run the example on the board:
```
cargo run --release
```
## Run the Example

Go back to the terminal window running the mesh configuration client and do:  
```
[mesh-cfgclient]# discover-unprovisioned on
```
This will scan for new devices and will generate output similar to:
```
    Scan result:
    rssi = -36
    UUID = SOMEINCREDIBLYLARGEHEXID
    OOB = A040
```
To end the scanning and bind the discovered node to the mesh network, do:

```
[mesh-cfgclient]# discover unprovisioned off
[mesh-cfgclient]# provision SOMEINCREDIBLYLARGEHEXID
```
Which then generates the output:
```
    Provisioning started
    Assigning address for 1 elements
    Provisioning done:
    Mesh node:
    UUID = SOMEINCREDIBLYLARGEHEXID
    primary = 00aa

    elements(1): 
```
You can now play with the given example or simply exit and end the client.

### Remarks
I didn't erase the board before staring the setup, so the nrf-softdevice already was on the chip.

#### Sources for Drogue.Device:
https://book.drogue.io/drogue-device/dev/getting_started.html  
https://github.com/drogue-iot/drogue-device/tree/main/examples/nrf52/nrf52840-dk/bt-mesh  
https://book.drogue.io/drogue-device/dev/examples/nrf52/nrf52840-dk/bt-mesh/README.html  
https://embedded-trainings.ferrous-systems.com/troubleshoot-cargo-flash.html  