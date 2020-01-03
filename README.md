# rfm9x-rs
Platform-agnostic driver for the [Hoperf RFM95/96/97/98][device-info]
RFM95/96/97/98 Low Power Long Range Transceiver Module driver which uses SPI via `embedded-hal`.
Usable via any compatible board crate (e.g. [trellis_m4]).

[Documentation][docs-link]

## Requirements

- Rust 1.32+
- `embedded-hal` SPI driver

## Code of Conduct

We abide by the [Contributor Covenant][cc] and ask that you do as well.

For more information, please see [CODE_OF_CONDUCT.md].

## TODO

### LoRa
- [ ] Register mapping
- [ ] Mode setting
- [ ] Modem config setup
- [ ] Send packets
- [ ] CRC header in send packets
- [ ] Modem status
- [ ] Rx modem setup
- [ ] FIFO Rx

### FSM/OSK
- [ ] Register mapping
- [ ] Mode setting
- [ ] Modem config setup
- [ ] Send packets
- [ ] Modem status
- [ ] Rx modem setup
- [ ] FIFO Rx
 
## License

Copyright © 2019 PathfinderZA Software (PTY) LTD

Dual licensed under your choice of either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[//]: # (badges)

[//]: # (general links)

[device-info]: https://www.hoperf.com/modules/lora/RFM95TW.html
[trellis_m4]: https://crates.io/crates/trellis_m4
[cc]: https://contributor-covenant.org
[CODE_OF_CONDUCT.md]: https://github.com/mvniekerk/rfm9x-rs/blob/develop/CODE_OF_CONDUCT.md
