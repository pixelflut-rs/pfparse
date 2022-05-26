use pfparse;

#[test]
fn test_from_color() {
    let c: u32 = pfparse::color::Color {
        r: 15,
        g: 163,
        b: 177,
        a: 255,
    }
    .into();

    assert_eq!("0FA3B1FF", format!("{:0>8X?}", c).as_str());
    assert_eq!(c, 262386175)
}

#[test]
fn test_from_u32() {
    let c = pfparse::color::Color::from(262386175);

    assert_eq!(
        c,
        pfparse::color::Color {
            r: 15,
            g: 163,
            b: 177,
            a: 255,
        }
    )
}
