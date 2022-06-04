use pfparse;

#[test]
fn test_basic() {
    let r = match pfparse::parse("SIZE") {
        Ok(r) => r,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(r, pfparse::Command::Size);
}

#[test]
fn test_rest() {
    let r = match pfparse::parse("SIZE foo") {
        Ok(r) => r,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(r, pfparse::Command::Size);
}
