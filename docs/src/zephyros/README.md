# ZepyhrOS

Zephyr is a small real-time operating system for connected, resource-constrained and embedded devices written in C.
For us the most interesting parts are the [`nrf52840dk`](https://docs.zephyrproject.org/latest/boards/arm/nrf52840dk_nrf52840/doc/index.html) and [BLE support](https://docs.zephyrproject.org/latest/connectivity/bluetooth/bluetooth-arch.html) in the context of [Rust](https://github.com/tylerwhall/zephyr-rust).

## [😄] Bluetooth Status

[Zephyr's Bluetooth Stack](https://docs.zephyrproject.org/latest/connectivity/bluetooth/overview.html) is highly configurable and Bluetooth v5.3 compliant.
Support for all combinations of Host and Controllers is provided and the stack generally is Bluetooth-SIG qualified. 

## [☹️] Rust Status

There is no official Rust support for Zephyr. 

We in particular looked at [tylerwhall/zephyr-rust](https://github.com/tylerwhall/zephyr-rust) which makes it possible to write Rust code for Zephyr but lacks any support for Bluetooth features.


## [😄] Renode Status

The [main demonstrations](https://renode.readthedocs.io/en/latest/tutorials/ble-simulation.html) of Renode's BLE capabilities is done on Zepyhrs BLE examples and [documented here](https://zephyrproject.org/developing-and-testing-bluetooth-low-energy-products-on-nrf52840-in-renode-and-zephyr/).
This is for C code though. As at the time of writing there are no Rust bindings for the Bluetooth stack we can not evaluate the Rust side of this too deeply.

Our experiments with [tylerwhall/zephyr-rust](https://github.com/tylerwhall/zephyr-rust) showed that it works on their example projects and building them for the `nrf52840dk_nrf52840`.

![](sample.png)

The crash is according to their README intended. For the serial output to show in Renode the `repl` for the `nrf52840` needs to be modified so that easyDMA is off for the UART ports:

```
uart0: UART.NRF52840_UART @ sysbus 0x40002000
    easyDMA: false
    -> nvic@2

uart1: UART.NRF52840_UART @ sysbus 0x40028000
    easyDMA: false
    -> nvic@40
```