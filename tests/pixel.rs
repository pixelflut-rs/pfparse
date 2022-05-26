use pfparse;

#[test]
fn test_get() {
    let r = match pfparse::parse("PX 10 10") {
        Ok(r) => r,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(
        r,
        (
            "",
            pfparse::Command::Pixel {
                is_req: true,
                x: 10,
                y: 10,
                c: pfparse::color::BLACK,
            }
        )
    );
}

#[test]
fn test_invalid_get() {
    match pfparse::parse("PX 10") {
        Ok(_) => panic!("This should not parse"),
        Err(_) => {}
    };
}

#[test]
fn test_set_white() {
    let r = match pfparse::parse("PX 10 10 FFFFFF") {
        Ok(r) => r,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(
        r,
        (
            "",
            pfparse::Command::Pixel {
                is_req: false,
                x: 10,
                y: 10,
                c: pfparse::color::WHITE,
            }
        )
    )
}

#[test]
fn test_set_cyan() {
    let r = match pfparse::parse("PX 10 10 0FA3B1") {
        Ok(r) => r,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(
        r,
        (
            "",
            pfparse::Command::Pixel {
                is_req: false,
                x: 10,
                y: 10,
                c: pfparse::color::Color {
                    r: 15,
                    g: 163,
                    b: 177,
                    a: 255
                },
            }
        )
    )
}

#[test]
fn test_invalid_set() {
    match pfparse::parse("PX 10 10 FF") {
        Ok(r) => panic!("This should not parse: {:?}", r),
        Err(_) => {}
    };
}
