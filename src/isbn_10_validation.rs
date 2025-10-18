// https://www.codewars.com/kata/51fc12de24a9d8cb0e000001/train/rust
fn valid_isbn10(isbn: &str) -> bool {
    // it should be 10 characters long...
    if isbn.len() != 10 {
        return false;
    }
    let last_char = isbn.chars().last().unwrap();
    if is_invalid_last_character(last_char) {
        return false;
    }
    if are_any_characters_not_digits(isbn) {
        return false;
    }
    let mut validation_number = 0;
    for (idx, value) in isbn.chars().enumerate() {
        println!("Index: {}, Value: {}", idx, value);
        let character_number = if value == 'X' {
            10
        } else {
            value.to_digit(10).unwrap()
        };
        validation_number += character_number * (idx as u32 + 1)
    }
    validation_number % 11 == 0
}

fn are_any_characters_not_digits(isbn: &str) -> bool {
    !isbn.chars().take(9).all(|c| c.is_ascii_digit())
}

fn is_invalid_last_character(last_char: char) -> bool {
    last_char != 'X' && !last_char.is_ascii_digit()
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::valid_isbn10;

    fn dotest(isbn: &str, expected: bool) {
        let actual = valid_isbn10(isbn);
        assert!(
            actual == expected,
            "Test failed with isbn = {isbn}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest("1112223339", true);
        dotest("048665088X", true);
        dotest("1293000000", true);
        dotest("1234554321", true);
        dotest("1234512345", false);
        dotest("1293", false);
        dotest("X123456788", false);
        dotest("ABCDEFGHIJ", false);
        dotest("XXXXXXXXXX", false);
        dotest("123456789T", false);
    }
}
