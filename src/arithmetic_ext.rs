use crate::Decimal;
use crate::Error;
use overload::overload;
use std::cmp::Ordering;
use std::ops;

#[rustfmt::skip]
macro_rules! impl_try_from_primitive {
    ($TFrom:ty, $conversion_fn:path $(, $err:expr)?) => {
        #[doc = concat!(
            "Try to convert a `",
            stringify!($TFrom),
            "` into a `Decimal`.\n\nCan fail if the value is out of range for `Decimal`."
        )]
        impl TryFrom<$TFrom> for Decimal {
            type Error = crate::Error;

            #[inline]
            fn try_from(t: $TFrom) -> Result<Self, Error> {
                $conversion_fn(&t) $( .ok_or_else(|| $err) )?
            }
        }
    };
}

impl_try_from_primitive!(String, core::str::FromStr::from_str);
impl_try_from_primitive!(&String, core::str::FromStr::from_str);

macro_rules! overload_all_op {
    ($a:ty, $b:ty) => {
      overload!((a: ?$a) + (b: $b) -> Decimal { a + Decimal::try_from(b).unwrap() });
      overload!((a: ?$a) - (b: $b) -> Decimal { a - Decimal::try_from(b).unwrap() });
      overload!((a: ?$a) * (b: $b) -> Decimal { a * Decimal::try_from(b).unwrap() });
      overload!((a: ?$a) / (b: $b) -> Decimal { a / Decimal::try_from(b).unwrap() });
      overload!((a: ?$a) % (b: $b) -> Decimal { a % Decimal::try_from(b).unwrap() });
      overload!((a: &mut $a) += (b: $b) { a.add_assign(Decimal::try_from(b).unwrap()) });
      overload!((a: &mut $a) -= (b: $b) { a.sub_assign(Decimal::try_from(b).unwrap()) });
      overload!((a: &mut $a) *= (b: $b) { a.mul_assign(Decimal::try_from(b).unwrap()) });
      overload!((a: &mut $a) /= (b: $b) { a.div_assign(Decimal::try_from(b).unwrap()) });
      overload!((a: &mut $a) %= (b: $b) { a.rem_assign(Decimal::try_from(b).unwrap()) });
    };
}

overload_all_op!(Decimal, &str);
overload_all_op!(Decimal, &String);
overload_all_op!(Decimal, String);
overload_all_op!(Decimal, isize);
overload_all_op!(Decimal, i8);
overload_all_op!(Decimal, i16);
overload_all_op!(Decimal, i32);
overload_all_op!(Decimal, i64);
overload_all_op!(Decimal, i128);
overload_all_op!(Decimal, usize);
overload_all_op!(Decimal, u8);
overload_all_op!(Decimal, u16);
overload_all_op!(Decimal, u32);
overload_all_op!(Decimal, u64);
overload_all_op!(Decimal, u128);
overload_all_op!(Decimal, f32);
overload_all_op!(Decimal, f64);

macro_rules! impl_decimal_eq {
    ($type:ty) => {
        impl PartialEq<$type> for Decimal {
            fn eq(&self, other: &$type) -> bool {
                *self == Decimal::try_from(*other).unwrap()
            }
        }
    };
}

impl PartialEq<str> for Decimal {
    fn eq(&self, other: &str) -> bool {
        *self == Decimal::try_from(other).unwrap()
    }
}

impl PartialEq<&str> for Decimal {
    fn eq(&self, other: &&str) -> bool {
        *self == Decimal::try_from(*other).unwrap()
    }
}

impl PartialEq<String> for Decimal {
    fn eq(&self, other: &String) -> bool {
        *self == Decimal::try_from(other).unwrap()
    }
}

impl PartialEq<&String> for Decimal {
    fn eq(&self, other: &&String) -> bool {
        *self == Decimal::try_from(*other).unwrap()
    }
}

impl_decimal_eq!(isize);
impl_decimal_eq!(i8);
impl_decimal_eq!(i16);
impl_decimal_eq!(i32);
impl_decimal_eq!(i64);
impl_decimal_eq!(i128);
impl_decimal_eq!(usize);
impl_decimal_eq!(u8);
impl_decimal_eq!(u16);
impl_decimal_eq!(u32);
impl_decimal_eq!(u64);
impl_decimal_eq!(u128);
impl_decimal_eq!(f32);
impl_decimal_eq!(f64);

macro_rules! impl_decimal_ord {
    ($type:ty) => {
        impl PartialOrd<$type> for Decimal {
            fn partial_cmp(&self, other: &$type) -> Option<Ordering> {
                self.partial_cmp(&Decimal::try_from(*other).unwrap())
            }
        }
    };
}

impl PartialOrd<str> for Decimal {
    fn partial_cmp(&self, other: &str) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&Decimal::try_from(other).unwrap())
    }
}

impl PartialOrd<&str> for Decimal {
    fn partial_cmp(&self, other: &&str) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&Decimal::try_from(*other).unwrap())
    }
}

impl PartialOrd<String> for Decimal {
    fn partial_cmp(&self, other: &String) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&Decimal::try_from(other).unwrap())
    }
}

impl PartialOrd<&String> for Decimal {
    fn partial_cmp(&self, other: &&String) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&Decimal::try_from(*other).unwrap())
    }
}

impl_decimal_ord!(isize);
impl_decimal_ord!(i8);
impl_decimal_ord!(i16);
impl_decimal_ord!(i32);
impl_decimal_ord!(i64);
impl_decimal_ord!(i128);
impl_decimal_ord!(usize);
impl_decimal_ord!(u8);
impl_decimal_ord!(u16);
impl_decimal_ord!(u32);
impl_decimal_ord!(u64);
impl_decimal_ord!(u128);
impl_decimal_ord!(f32);
impl_decimal_ord!(f64);
