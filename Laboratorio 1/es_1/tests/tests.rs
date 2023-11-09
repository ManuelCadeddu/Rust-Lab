use es_1::slug::slugify;
#[test]
fn test_more_words() {
    assert_eq!(slugify("Ciao mi chiamo Gigi e tu?"), "ciao-mi-chiamo-gigi-e-tu");
}

#[test]
fn test_accented_characters() {
    assert_eq!(slugify("Gigi èè.."), "gigi-ee");
}

#[test]
fn test_invalid_characters() {
    assert_eq!(slugify("???(   )&&     "), "-");
}