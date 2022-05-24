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
                c: 0
            }
        )
    );
}

#[test]
fn test_set() {
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
                c: 0
            }
        )
    )
}
