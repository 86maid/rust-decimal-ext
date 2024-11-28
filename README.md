# rust_decimal_ext &emsp; [![Latest Version]][crates.io] [![Docs Badge]][docs]

[actions]: https://actions-badge.atrox.dev/paupino/rust-decimal/goto

[Latest Version]: https://img.shields.io/crates/v/rust-decimal.svg

[crates.io]: https://crates.io/crates/rust-decimal-ext

[Docs Badge]: https://docs.rs/rust_decimal/badge.svg

[docs]: https://docs.rs/rust_decimal_ext

`rust_decimal_ext` is an extension library that adds more operator trait support to [rust_decimal](https://crates.io/crates/rust-decimal). With this library, you can conveniently perform a variety of arithmetic operations, such as addition, subtraction, multiplication, and division, on `Decimal` types. It also supports automatic conversion and operations with numbers, strings, and other types.

## Usage

```rust
use rust_decimal_ext::prelude::*;

let mut a: Decimal = Decimal::from(1) + 1;
let mut b = Decimal::from(1) + 2.5;
let mut c = Decimal::from(10) + "5.5";

a += 1;
b += 2.5;
c += "5.5";

assert!(a == 3);
assert!(b == 6);
assert!(c == "21");

assert!(a > 2);
assert!(b < 10);
assert!(c >= "20");
assert!(c <= "21");

a.partial_cmp(&10).unwrap();
a.partial_cmp(&10.0).unwrap();
a.partial_cmp("10").unwrap();

// painc
_ = Decimal::from(1) + f64::NAN;
_ = Decimal::from(1) + f64::INFINITY;
_ = Decimal::from(1) + f64::NEG_INFINITY;
```