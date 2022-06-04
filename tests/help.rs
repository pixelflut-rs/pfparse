use pfparse;

#[test]
fn test_basic() {
    let r = match pfparse::parse("HELP") {
        Ok(r) => r,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(r, pfparse::Command::Help);
}

#[test]
fn test_rest() {
    let r = match pfparse::parse("HELP foo") {
        Ok(r) => r,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(r, pfparse::Command::Help);
}
