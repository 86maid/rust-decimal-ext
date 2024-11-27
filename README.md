# rust_decimal_ext &emsp; [![Latest Version]][crates.io] [![Docs Badge]][docs]

[actions]: https://actions-badge.atrox.dev/paupino/rust-decimal/goto

[Latest Version]: https://img.shields.io/crates/v/rust-decimal.svg

[crates.io]: https://crates.io/crates/rust-decimal-ext

[Docs Badge]: https://docs.rs/rust_decimal/badge.svg

[docs]: https://docs.rs/rust_decimal_ext

`rust_decimal_ext` is an extension library that adds more operator trait support to [rust_decimal](https://crates.io/crates/rust-decimal). With this library, you can more conveniently perform arithmetic operations on `Decimal` types, supporting automatic conversion and addition with numbers, strings, and other types.

## Usage

```rust
use rust_decimal_ext::prelude::*;

let mut a = Decimal::from(100) + 100;
let mut b = Decimal::from(100) + 3.14;
let mut c = Decimal::from(100) + "3.14";
let mut d = Decimal::from(100) + "3.14".to_string();

a += 100;
b += 3.14;
c += "3.14";
d += "3.14";

assert!(a == 300);
assert!(b == 106.28);
assert!(c == "106.28");
assert!(d == "106.28");
```