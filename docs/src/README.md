# Are We IoT Yet?

_The state of the Internet of Things in Rust._


With a focus on Bluetooth Low Energy (BLE), pure Rust programming, and simulation support on edge-devices, we are **not** IoT yet.

## Status

| OS        | Language | BLE Support | Application Rust Support | Renode Support |
|-----------|----------|-------------|--------------|----------------|
| [RIOT](riot)      | C        |     😄      |       🙂      |        😄      |
| [tock](tock)      | Rust     |      😐     |       🙂      |        😄      |
| [Embassy](embassy)   | Rust     |      😄     |      😄       |       ☹️       |
| [Drogue IoT](drogueiot) | Rust     |      😄     |      😄       |      ☹️        |
| [Zephyr](zephyr)    | C        |     😄      |       ☹️      |       😄       |

## Trade-offs 

- BLE and pure Rust programming are achievable using [Embassy](embassy) and [DrogueIOT](drogueiot) but simulation support is missing.
- BLE and simulation support using [Renode](introduction/renode) while programming mainly in Rust is achievable with [RIOT](riot) but requires proficiency in C.
- Pure Rust programming and simulation support using [Renode](introduction/renode) work with [tock](tock) but only provide very limited BLE support.

## Open Tasks

We identified the following open tasks that would help make Rust IoT-ready:

- [Add Renode support for Embassy and Drogue IoT](embassy/renode)
- [Improve the RIOT rust wrappers and add support for NimBLE](https://github.com/RIOT-OS/rust-riot-wrappers)
- [Have more complete and maintained Zephyr Rust support](https://github.com/tylerwhall/zephyr-rust)
- [Build a full Bluetooth stack for tock](https://github.com/tock/tock)


