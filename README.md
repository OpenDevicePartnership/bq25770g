# BQ2577x Rust Device Driver

A `#[no_std]` platform-agnostic driver for the
[BQ2577x](https://www.ti.com/lit/gpn/bq25770g) line of buck-boost
battery charge controllers to charge 2- to 5-cell battery. The charger
supports several input supplies, including USB adapter, extended power
range (EPR) USB-C Power Delivery (PD), standard power range (SPR)
USB-c Power Delivery (PD), and traditional adapters.

The BQ25770G device responds to address =0x09=. The driver is written
in a way such that the user need not worry about the I2C device
address; the driver takes care of that internally. Every register has
been converted to sensible Rust types to avoid situations where
invalid values are written to registers.

A higher level API will be built on top of the lower level register
accessor in order to produce a more ergonomic API for the user of the
driver.

## MSRV

Currently, rust `1.79` and up is supported, but some previous versions
may work.

## License

Licensed under the terms of the [MIT license](http://opensource.org/licenses/MIT).

## Contribution

Unless you explicitly state otherwise, any contribution submitted for
inclusion in the work by you shall be licensed under the terms of the
MIT license.

License: MIT

