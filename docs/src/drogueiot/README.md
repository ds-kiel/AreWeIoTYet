# Drogue IoT


## Setup Drogue-IoT / Drogue-Device

### Terminal Fenster 1
1. ```git clone https://github.com/drogue-iot/drogue-device.git``` 
2. ```cd drogue-device```
3. On Ubuntu 22.04: 
    - Check if ```bluez``` and ```bluez-meshd``` are already installed
    - If not installed, run:
    ```sud apt install bluez bluez-meshd```  
4. ```sudo service bluetooth stop```
5. ```sudo /usr/lib/bluetooth/bluetooth-meshd -nd --debug```

### Terminal Fenster 2
6. ```mesh-cfgclient```
7. If it says config_db.json not found:  
```[mesh-cfgclient]# create```

### Terminal Fenster 3
8. ```cd <drogue-device-git>/examples/nrf52/nrf52840-dk/bt-mesh```
9. Try: ```DEFMT_LOG=debug cargo run --release```
    - I am getting the error message ```rust-lld error: undefined symbol: _defmt_acquire```
10. Try: ```cargo run --release```
    - I am getting the error message ```USB error while opening USB device: Access denied (insufficient permissions)```
11. USB Permissions on Ubuntu 22.04:   
    1. ```lsusb```
    2. ```lsusb -d <id for SEGGER J-Link```
    3. ```ls -l /dev/bus/usb/<bus-nr>/<device-nr>```   
    Outputs for me: ```crw-rw-r-- 1 root root 189, 8 Jul 18 06:48```
    4. ```sudo gedit /etc/udev/rules.d/50-oxidize-global.rules```   
    File Content:
    ```
    # udev rules to allow access to USB devices as a non-root user

    # nRF52840 Dongle in bootloader mode
    ATTRS{idVendor}=="1915", ATTRS{idProduct}=="521f", TAG+="uaccess"

    # nRF52840 Dongle applications
    ATTRS{idVendor}=="2020", TAG+="uaccess"

    # nRF52840 Development Kit
    ATTRS{idVendor}=="1366", ENV{ID_MM_DEVICE_IGNORE}="1", TAG+="uaccess"
    ```
    5. ```sudo udevadm control --reload-rules```
    6. Optional: Repeat steps 11.1 to 11.3 to check if ```crw-rw-w-- 1``` changed to ```crw-rw-r--+ 1```
12. Try ```cargo run --release```
    Outputs for me: ```(HOST) INFO success!```

### Back to Terminal Fenster 2 (starting to play)
13. ```[mesh-cfgclient]# discover-unprovisioned on```  
    Outputs for me: 
    ```
    Scan result:
    rssi = -36
    UUID = SOMEINCREDIBLYLARGEHEXID
    OOB = A040
    ```
14. ```[mesh-cfgclient]# discover unprovisioned off```
15. ```[mesh-cfgclient]# provision SOME INCREDIBLYLARGEHEXID```  
    Outputs for me:
    ```
    Provisioning started
    Assigning address for 1 elements
    Provisioning done:
    Mesh node:
    UUID = SOMEINCREDIBLYLARGEHEXID
    primary = 00aa

    elements(1): 
    ```
16. ```[mesh-cfgclient]# target 00aa```  
    Outputs for me: ```Configuring node 00aa```
17. ```beacon-get```  
    Outputs for me: 
    ```
    Received BeaconStatus (len 1)
    Node 00aa: Config Beacon Status 0x01
    ```
18. Try: ```[config: Target = 00aa]# node-reset```  
Outputs for me: Nothing
19. Try ```[config: Target = 00aa]# back```  
    Outputs for me: 
    ```
    Menu main:
    Available commands: 
    -------------------
    config
    ..
    ```
20. Try ```[config: Target = 00aa]# back``` as the first part has not changed yet  
Outputs for me:
```Invalid command in menu main: back```
21. ```[config: Target = 00aa]# exit```   
    Ends the mesh-cfgclient


#### Sourcs for dependencies:
Setup-embassy.pdf  
I didn't erase the board, so the nrf-sfotdevice already was on the chip working

#### Sources for Drogue.Device:
https://book.drogue.io/drogue-device/dev/getting_started.html  
https://github.com/drogue-iot/drogue-device/tree/main/examples/nrf52/nrf52840-dk/bt-mesh  
https://book.drogue.io/drogue-device/dev/examples/nrf52/nrf52840-dk/bt-mesh/README.html  
https://embedded-trainings.ferrous-systems.com/troubleshoot-cargo-flash.html  