# Examples
The nrf-softdevice repository includes multiple examples. Their source code can be found in `examples/src/bin`.

### BLE Advertise
This example spawns BLE advertises with the device name "HelloRust". If you want to modify the name, make sure to modify the hardcoded string in the `adv` object.  

### BLE BaS Central  
```
cargo run --bin ble_bas_central
```
This example provides a [GATT server](https://learn.adafruit.com/introduction-to-bluetooth-low-energy/gatt) with a Battery Service. The central device can read and write the battery status.

### BLE BaS Peripheral
```
cargo run --bin ble_bas_peripheral
```
This example provides a [GATT server](https://learn.adafruit.com/introduction-to-bluetooth-low-energy/gatt) with a Battery Service and a Foo Service. The peripheral device sends notifications on its battery status.

### BLE Bond Peripheral
```
cargo run --bin ble_bond_peripheral
```
This example provides a [GATT server](https://learn.adafruit.com/introduction-to-bluetooth-low-energy/gatt) with a Battery Service. The more interesting part is the use of a bonder with its security handler. This leads to displaying a passkey on connection.

### BLE DIS BaS Peripheral Builder
```
cargo run --bin ble_dis_bas_peripheral_builder
```
This example provides a [GATT server](https://learn.adafruit.com/introduction-to-bluetooth-low-energy/gatt) with a Device Information Service and a Battery Service.

### BLE L2CAP Central
```
cargo run --bin ble_l2cap_central
```
This example uses the L2CAP on top of the BLE connection. This node is transmitting data.

### BLE L2CAP Peripheral
```
cargo run --bin ble_l2cap_peripheral
```
This example uses the L2CAP on top of the BLE conection. This node is receiving data.

### BLE Peripheral onoff
```
cargo run --bin ble_peripheral_onoff
```
This example makes use of the buttons on the board. Press button 1 to enable BLE or press button 2 to disable BLE. On connection the device creates a [GATT server](https://learn.adafruit.com/introduction-to-bluetooth-low-energy/gatt) with a Foo Service.

### BLE Scan
```
cargo run --bin ble_scan
```
This example scans for BLE devices. It reports the type or the discovered devices as info text.

### Flash
```
cargo run --bin flash
```
This example erases the application from the board but should leave the softdevice on it.