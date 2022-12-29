use chess::square::Square;

#[test]
fn test_from_an() {
    assert_eq!(Square::from_an("a1").unwrap().0, 0);
    assert_eq!(Square::from_an("a2").unwrap().0, 8);
    assert_eq!(Square::from_an("e4").unwrap().0, 28);
    assert_eq!(Square::from_an("h8").unwrap().0, 63);
}

#[test]
fn test_from_an_err() {
    assert!(Square::from_an("a0").is_err());
    assert!(Square::from_an("i1").is_err());
    assert!(Square::from_an("").is_err());
    assert!(Square::from_an("e44").is_err());
    assert!(Square::from_an("ee").is_err());
    assert!(Square::from_an("44").is_err());
}
