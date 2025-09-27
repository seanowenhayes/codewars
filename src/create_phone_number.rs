fn numbers_to_string(numbers: &[u8]) -> String {
    numbers
        .iter()
        .map(|&c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
}
fn create_phone_number(numbers: &[u8]) -> String {
    let area_code = numbers_to_string(&numbers[0..3]);
    let first_bit = numbers_to_string(&numbers[3..6]);
    let last_bit = numbers_to_string(&numbers[6..]);
    format!("({}) {}-{}", area_code, first_bit, last_bit)
}

#[test]
fn returns_expected() {
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
        "(123) 456-7890"
    );
    assert_eq!(
        create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
        "(111) 111-1111"
    );
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
        "(123) 456-7899"
    );
}
