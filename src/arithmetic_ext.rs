use crate::Decimal;
use crate::Error;
use overload::overload;
use std::cmp::Ordering;

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

macro_rules! overload_all_op_right {
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

macro_rules! overload_all_op_left {
    ($a:ty, $b:ty) => {
        overload!((a: $a) + (b: ?$b) -> Decimal { Decimal::try_from(a).unwrap() + b });
        overload!((a: $a) - (b: ?$b) -> Decimal { Decimal::try_from(a).unwrap() - b });
        overload!((a: $a) * (b: ?$b) -> Decimal { Decimal::try_from(a).unwrap() * b });
        overload!((a: $a) / (b: ?$b) -> Decimal { Decimal::try_from(a).unwrap() / b });
        overload!((a: $a) % (b: ?$b) -> Decimal { Decimal::try_from(a).unwrap() % b });
        overload!((a: &mut $a) += (b: $b) {
            *a = (*a + b).try_into().unwrap()
        });
        overload!((a: &mut $a) -= (b: $b) {
            *a = (*a - b).try_into().unwrap()
        });
        overload!((a: &mut $a) *= (b: $b) {
            *a = (*a * b).try_into().unwrap()
        });
        overload!((a: &mut $a) /= (b: $b) {
            *a = (*a / b).try_into().unwrap()
        });
        overload!((a: &mut $a) %= (b: $b) {
            *a = (*a % b).try_into().unwrap()
        });
    };
}

overload_all_op_right!(Decimal, &str);
overload_all_op_right!(Decimal, String);
overload_all_op_right!(Decimal, isize);
overload_all_op_right!(Decimal, i8);
overload_all_op_right!(Decimal, i16);
overload_all_op_right!(Decimal, i32);
overload_all_op_right!(Decimal, i64);
overload_all_op_right!(Decimal, i128);
overload_all_op_right!(Decimal, usize);
overload_all_op_right!(Decimal, u8);
overload_all_op_right!(Decimal, u16);
overload_all_op_right!(Decimal, u32);
overload_all_op_right!(Decimal, u64);
overload_all_op_right!(Decimal, u128);
overload_all_op_right!(Decimal, f32);
overload_all_op_right!(Decimal, f64);

overload!((a: &str) + (b: ?Decimal) -> Decimal { Decimal::try_from(a).unwrap() + b });
overload!((a: &str) - (b: ?Decimal) -> Decimal { Decimal::try_from(a).unwrap() - b });
overload!((a: &str) * (b: ?Decimal) -> Decimal { Decimal::try_from(a).unwrap() * b });
overload!((a: &str) / (b: ?Decimal) -> Decimal { Decimal::try_from(a).unwrap() / b });
overload!((a: &str) % (b: ?Decimal) -> Decimal { Decimal::try_from(a).unwrap() % b });
overload!((a: String) + (b: ?Decimal) -> Decimal { Decimal::try_from(a).unwrap() + b });
overload!((a: String) - (b: ?Decimal) -> Decimal { Decimal::try_from(a).unwrap() - b });
overload!((a: String) * (b: ?Decimal) -> Decimal { Decimal::try_from(a).unwrap() * b });
overload!((a: String) / (b: ?Decimal) -> Decimal { Decimal::try_from(a).unwrap() / b });
overload!((a: String) % (b: ?Decimal) -> Decimal { Decimal::try_from(a).unwrap() % b });
overload_all_op_left!(isize, Decimal);
overload_all_op_left!(i8, Decimal);
overload_all_op_left!(i16, Decimal);
overload_all_op_left!(i32, Decimal);
overload_all_op_left!(i64, Decimal);
overload_all_op_left!(i128, Decimal);
overload_all_op_left!(usize, Decimal);
overload_all_op_left!(u8, Decimal);
overload_all_op_left!(u16, Decimal);
overload_all_op_left!(u32, Decimal);
overload_all_op_left!(u64, Decimal);
overload_all_op_left!(u128, Decimal);
overload_all_op_left!(f32, Decimal);
overload_all_op_left!(f64, Decimal);

macro_rules! impl_decimal_eq {
    ($type:ty) => {
        impl PartialEq<$type> for Decimal {
            #[track_caller]
            fn eq(&self, other: &$type) -> bool {
                *self == Decimal::try_from(*other).unwrap()
            }
        }

        impl PartialEq<Decimal> for $type {
            #[track_caller]
            fn eq(&self, other: &Decimal) -> bool {
                *other == *self
            }
        }
    };
}

impl PartialEq<str> for Decimal {
    #[track_caller]
    fn eq(&self, other: &str) -> bool {
        *self == Decimal::try_from(other).unwrap()
    }
}

impl PartialEq<&str> for Decimal {
    #[track_caller]
    fn eq(&self, other: &&str) -> bool {
        *self == Decimal::try_from(*other).unwrap()
    }
}

impl PartialEq<String> for Decimal {
    #[track_caller]
    fn eq(&self, other: &String) -> bool {
        *self == Decimal::try_from(other).unwrap()
    }
}

impl PartialEq<&String> for Decimal {
    #[track_caller]
    fn eq(&self, other: &&String) -> bool {
        *self == Decimal::try_from(*other).unwrap()
    }
}

impl PartialEq<Decimal> for str {
    #[track_caller]
    fn eq(&self, other: &Decimal) -> bool {
        *other == *self
    }
}

impl PartialEq<Decimal> for &str {
    #[track_caller]
    fn eq(&self, other: &Decimal) -> bool {
        *other == *self
    }
}

impl PartialEq<Decimal> for String {
    #[track_caller]
    fn eq(&self, other: &Decimal) -> bool {
        *other == *self
    }
}

impl PartialEq<Decimal> for &String {
    #[track_caller]
    fn eq(&self, other: &Decimal) -> bool {
        *other == *self
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
                self.partial_cmp(&Decimal::try_from(*other).ok()?)
            }
        }

        impl PartialOrd<Decimal> for $type {
            fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
                Decimal::try_from(*self).ok()?.partial_cmp(other)
            }
        }
    };
}

impl PartialOrd<str> for Decimal {
    fn partial_cmp(&self, other: &str) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&Decimal::try_from(other).ok()?)
    }
}

impl PartialOrd<&str> for Decimal {
    fn partial_cmp(&self, other: &&str) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&Decimal::try_from(*other).ok()?)
    }
}

impl PartialOrd<String> for Decimal {
    fn partial_cmp(&self, other: &String) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&Decimal::try_from(other).ok()?)
    }
}

impl PartialOrd<&String> for Decimal {
    fn partial_cmp(&self, other: &&String) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&Decimal::try_from(*other).ok()?)
    }
}

impl PartialOrd<Decimal> for str {
    fn partial_cmp(&self, other: &Decimal) -> Option<core::cmp::Ordering> {
        other.partial_cmp(self)
    }
}

impl PartialOrd<Decimal> for &str {
    fn partial_cmp(&self, other: &Decimal) -> Option<core::cmp::Ordering> {
        other.partial_cmp(self)
    }
}

impl PartialOrd<Decimal> for String {
    fn partial_cmp(&self, other: &Decimal) -> Option<core::cmp::Ordering> {
        other.partial_cmp(self)
    }
}

impl PartialOrd<Decimal> for &String {
    fn partial_cmp(&self, other: &Decimal) -> Option<core::cmp::Ordering> {
        other.partial_cmp(self)
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
