#[test]
fn test_overload() {
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
}
