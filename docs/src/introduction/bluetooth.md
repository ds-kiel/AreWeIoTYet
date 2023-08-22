# Bluetooth Stack Comparison
| Chacteristics on BLE Stack | Embassy | Drogue | TockOS | RiotOS | ZephyrOS | FreeRTOS |  
|---|---|---|---|---|---|---|  
| Number of concurrent links | 20 | 20 | - | 32 (NimBLE) | unlimited | 20-32 |
| Advertising (Broadcaster) | SoftDevice | SoftDevice | TockOS Stack | NimBLE/ Skald | Zephyr Stack | SoftDevice/ NimBLE |
|Connecting (central) | SoftDevice | SoftDevice |-| NimBLE | ZephyrStack | SoftDevice/NimBLE |
| Scanning (Observer) | SoftDevice | SoftDevice | TockOS Stack | NimBLE | Zephyr Stack | SoftDevice/ NimBLE|
|Connectable (peripheral) | SoftDevice | SoftDevice |-| NimBLE | ZephyrStack | SoftDevice/NimBLE |
Over-air device firmware updates | SoftDevice | SoftDevice |-|-|-|-|
| Asynchronous, event-driven behaviour | SoftDevice | SoftDevice |-|-|-|-|
|Implemented in Rust |-|-| TockOS Stack |-|-|-|
| Implemented in C | SoftDevice | SoftDevice |-| NimBLE | ZephyrStack | SoftDevice/ NimBLE |
| Bluetooth Mesh | (SoftDevice) | (SoftDevice) |-| NimBLE | ZephyrStack | NimBLE |
L2CAP Connections | SoftDevice | SoftDevice |-| NimBLE | ZephyrStack | SoftDevice/ NimBLE |