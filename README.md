# chinese-format

_Convert data types to Chinese, with Rust_

[![Crates.io Version](https://img.shields.io/crates/v/chinese-format?style=for-the-badge&logo=rust)
](https://crates.io/crates/chinese-format)

![Logo](docs/logo.svg)

This crate focuses on converting data types to `Chinese`,
which can be achieved by implementing the `ChineseFormat` trait.

As a consequence, the library provides:

- **ready-made conversions** for standard data types (integers,
  strings, `Option`, pairs, ...) - but also for fairly specific types like `DigitSequence`.

- **Gregorian date/time**, in the `gregorian` module, in different formats via `DateBuilder`, `LinearTime` and `DeltaTime`.

- **Monetary units**, in the `currency` module - at present, `RenminbiCurrency`(人民币).

- **Dedicated numeric types** - such as `Decimal`, `Fraction` and `Sign`.

- the `ChineseVec` sequence, to simplify the manipulation of _arbitrary
  chains of logograms_, as well as **placeholders**.

- the `Measure` trait and its related macros - especially `define_measure`.

## Features

The crate supports the following _optional_ features:

- `digit-sequence`:

  - enables conversions to Chinese for [DigitSequence](https://crates.io/crates/digit-sequence).

  - enables the `Decimal` and `IntegerPart` types.

- `currency`: enables the whole `currency` module for monetary conversions.

- `gregorian`: enables the `gregorian` module for date/time conversions.

  _Also enables_: `digit-sequence`.

## Crates.io

https://crates.io/crates/chinese-format

## Documentation

https://docs.rs/chinese-format

## License

[MIT](LICENSE)
