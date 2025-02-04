use jtoo::DateTimeOffset;

#[test]
fn millisecond() {
    assert_eq!(999, DateTimeOffset::MAX.millisecond());
}

#[test]
fn microsecond() {
    assert_eq!(999_999, DateTimeOffset::MAX.microsecond());
}
