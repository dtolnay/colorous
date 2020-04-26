use colorous::Color;

#[test]
fn test_hex() {
    let (r, g, b) = (10, 100, 255);
    let color = Color { r, g, b };
    assert_eq!("0a64ff", format!("{:x}", color));
    assert_eq!("0A64FF", format!("{:X}", color));
}
