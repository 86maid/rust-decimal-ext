# rust_decimal_ext &emsp; [![Latest Version]][crates.io] [![Docs Badge]][docs]

[Latest Version]: https://img.shields.io/crates/v/rust_decimal_ext.svg

[crates.io]: https://crates.io/crates/rust_decimal_ext

[Docs Badge]: https://docs.rs/rust_decimal_ext/badge.svg

[docs]: https://docs.rs/rust_decimal_ext

`rust_decimal_ext` is an extension library that adds more operator trait support to [rust_decimal](https://crates.io/crates/rust-decimal). With this library, you can conveniently perform a variety of arithmetic operations, such as addition, subtraction, multiplication, and division, on `Decimal` types. It also supports automatic conversion and operations with numbers, strings, and other types.

## Usage

```rust
use rust_decimal::prelude::*;

let mut a: Decimal = Decimal::from(1) + 1;
let mut b: Decimal = Decimal::from(2) + 0.5;
let mut c: Decimal = Decimal::from(5) + "0.5";

a += 1;
b += 0.5;
c += "0.5";

assert!(a == 3);
assert!(b == 3);
assert!(c == "6");

assert!(a > 2);
assert!(b < 4);
assert!(c >= "6");
assert!(c <= "6");

a.partial_cmp(&3).unwrap();
a.partial_cmp(&3.0).unwrap();
a.partial_cmp("3").unwrap();

assert!(a.partial_cmp("hello").is_none());
assert!(a.partial_cmp(&f64::NAN).is_none());
assert!(a.partial_cmp(&f64::INFINITY).is_none());
assert!(a.partial_cmp(&f64::NEG_INFINITY).is_none());

assert!(1 == Decimal::from(1));
assert!(Decimal::from(1) == 1);
assert!(1 + Decimal::from(1) == 2);
assert!("1" + Decimal::from(1) == Decimal::from(2));
assert!("1" == Decimal::from(1));
assert!("1".partial_cmp(&Decimal::from(1)).unwrap().is_eq());

let mut z = 1;
z += Decimal::from(1);
assert!(z == 2);

let mut z = Decimal::from(1);
z += 1;
assert!(z == 2);

// panic
_ = Decimal::from(1) + f64::NAN;
_ = Decimal::from(1) + f64::INFINITY;
_ = Decimal::from(1) + f64::NEG_INFINITY;
```