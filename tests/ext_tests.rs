#[test]
fn test_overload() {
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
}
