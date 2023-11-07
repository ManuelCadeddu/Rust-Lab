use es_1::slugify;
#[test]
fn test_add() {
    assert_eq!(slugify("Gigi èè.."), "gigi-ee");
}

