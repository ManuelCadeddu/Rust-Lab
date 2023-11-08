use es_1::slug::slugify;

#[test]
fn test_add() {
    assert_eq!(slugify("Gigi èè.."), "gigi-ee");
}