# chinese-format

_Convert data types to Chinese_

[![CI](https://github.com/giancosta86/chinese-format/actions/workflows/publish-to-crates.yml/badge.svg)](https://github.com/giancosta86/chinese-format/actions/workflows/publish-to-crates.yml)

This crate focuses on converting data types to `Chinese`,
which can be achieved by implementing the `ToChinese` trait.

As a consequence, it provides:

- **ready-made conversions** for standard data types such as integers,
  strings, ... - but also for fairly specific types like `DigitSequence`.

- **custom data types** - such as `Sign` and `Fraction`.

- the `Measure` trait and its related macros - especially `define_measure`.

- the `ChineseVec` sequence, to simplify the manipulation of _complex
  chains of logograms_.

## Optional features

- **digit-sequence**: enables conversions to Chinese for [DigitSequence](https://crates.io/crates/digit-sequence)

## Crates.io

https://crates.io/crates/chinese-format

## Documentation

https://docs.rs/chinese-format

## License

[MIT](LICENSE)