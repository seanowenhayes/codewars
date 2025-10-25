// https://www.codewars.com/kata/51b66044bce5799a7f000003/rust
// +--------+-------+
// | Symbol | Value |
// +--------+-------+
// |    M   |  1000 |
// |   CM   |   900 |
// |    D   |   500 |
// |   CD   |   400 |
// |    C   |   100 |
// |   XC   |    90 |
// |    L   |    50 |
// |   XL   |    40 |
// |    X   |    10 |
// |   IX   |     9 |
// |    V   |     5 |
// |   IV   |     4 |
// |    I   |     1 |
// +--------+-------+

use std::collections::HashMap;

const M: u32 = 1000;
const CM: u32 = 900;
const D: u32 = 500;
const CD: u32 = 400;
const C: u32 = 100;
const XC: u32 = 90;
const L: u32 = 50;
const XL: u32 = 40;
const X: u32 = 10;
const IX: u32 = 9;
const V: u32 = 5;
const IV: u32 = 4;
const I: u32 = 1;

pub fn to_roman(r: u32) -> String {
    let numerals: HashMap<u32, &str> = HashMap::from([
        (M, "M"),
        (CM, "CM"),
        (D, "D"),
        (CD, "CD"),
        (C, "C"),
        (XC, "XC"),
        (L, "L"),
        (XL, "XL"),
        (X, "X"),
        (IX, "IX"),
        (IV, "IV"),
        (V, "V"),
        (I, "I"),
    ]);
    let mut roman = String::new();
    let mut remainder = r;
    if r >= M {
        let times: usize = (r / M) as usize;
        roman.push_str("M".to_owned().repeat(times).as_str());
        remainder = remainder - (M * times as u32);
    }
    vec![CM, D, CD, C, XC, L, XL, X, IX, V, IV]
        .iter()
        .for_each(|roman_numeral| {
            while remainder >= *roman_numeral {
                roman.push_str(numerals.get(roman_numeral).expect("MISSING ROMAN NUMERAL"));
                remainder -= roman_numeral;
            }
        });
    let times: usize = remainder as usize;
    roman.push_str("I".to_owned().repeat(times).as_str());
    roman
}

pub fn from_roman(s: &str) -> u32 {
    let mut answer = 0;
    let mut input = s.to_string();
    let digits: HashMap<&str, u32> = HashMap::from([
        ("M", M),
        ("CM", CM),
        ("D", D),
        ("CD", CD),
        ("C", C),
        ("XC", XC),
        ("L", L),
        ("XL", XL),
        ("X", X),
        ("IX", IX),
        ("V", V),
        ("IV", IV),
        ("I", I),
    ]);
    while input.len() > 0 {
        for numeral in vec![
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ] {
            if input.starts_with(numeral) {
                let digit = digits.get(numeral).expect("Missing numeral!");
                answer += digit;
                input = input.replacen(numeral, "", 1);
            }
        }
    }
    answer
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod example_tests {
    use super::{from_roman, to_roman};

    fn assert_to_roman(r: u32, expected: &str) {
        let actual = to_roman(r);
        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right)"
        )
    }

    fn assert_from_roman(s: &str, expected: u32) {
        let actual = from_roman(s);
        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right)"
        )
    }

    #[test]
    fn _1_to_roman() {
        assert_to_roman(1775, "MDCCLXXV");
        assert_to_roman(1000, "M");
        assert_to_roman(4, "IV");
        assert_to_roman(1, "I");
        assert_to_roman(1990, "MCMXC");
        assert_to_roman(2008, "MMVIII");
    }

    #[test]
    fn _2_from_roman() {
        assert_from_roman("XXI", 21);
        assert_from_roman("I", 1);
        assert_from_roman("IV", 4);
        assert_from_roman("IX", 9);
        assert_from_roman("MMVIII", 2008);
        assert_from_roman("MDCLXVI", 1666);
    }
}
