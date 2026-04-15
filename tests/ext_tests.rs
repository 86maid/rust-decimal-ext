#[test]
fn test_overload() {
    use rust_decimal::prelude::*;

    let mut a: Decimal = Decimal::from(1) + 1;
    let mut b = Decimal::from(2) + 0.5;
    let mut c = Decimal::from(5) + "0.5";

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

    // panic
    _ = Decimal::from(1) + f64::NAN;
    _ = Decimal::from(1) + f64::INFINITY;
    _ = Decimal::from(1) + f64::NEG_INFINITY;
}
