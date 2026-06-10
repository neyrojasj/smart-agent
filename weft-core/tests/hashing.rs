use weft_core::canonical_hash;

#[test]
fn same_text_produces_same_eight_hex_char_hash() {
    let statement = "The system must allow users to log in.";
    let acceptance = vec![
        "Given valid credentials, the user is authenticated.".to_string(),
        "Given invalid credentials, an error is shown.".to_string(),
    ];

    let hash = canonical_hash(statement, &acceptance);

    assert_eq!(hash.len(), 8);
    assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    assert_eq!(hash, canonical_hash(statement, &acceptance));
}

#[test]
fn surrounding_whitespace_does_not_affect_hash() {
    let trimmed = canonical_hash(
        "The system must allow users to log in.",
        &["Given valid credentials, the user is authenticated.".to_string()],
    );

    let padded = canonical_hash(
        "  The system must allow users to log in.\n",
        &["  Given valid credentials, the user is authenticated.  ".to_string()],
    );

    assert_eq!(trimmed, padded);
}

#[test]
fn reordering_acceptance_criteria_changes_the_hash() {
    let statement = "The system must allow users to log in.";
    let original = vec![
        "Criterion A".to_string(),
        "Criterion B".to_string(),
    ];
    let reordered = vec![
        "Criterion B".to_string(),
        "Criterion A".to_string(),
    ];

    assert_ne!(
        canonical_hash(statement, &original),
        canonical_hash(statement, &reordered)
    );
}

#[test]
fn unicode_normalization_makes_equivalent_text_hash_the_same() {
    // "é" as a single precomposed code point (NFC) vs. "e" + combining acute accent (NFD).
    let nfc = canonical_hash("Caf\u{00e9}", &["item".to_string()]);
    let nfd = canonical_hash("Cafe\u{0301}", &["item".to_string()]);

    assert_eq!(nfc, nfd);
}
