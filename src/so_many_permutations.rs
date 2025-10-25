// https://www.codewars.com/kata/5254ca2719453dcc0b00027d/train/rust
//
//
use std::collections::HashSet;

pub fn permutations(s: &str) -> Vec<String> {
    if s.len() < 2 {
        return vec![s.to_owned()];
    }
    let mut mutations: HashSet<String> = HashSet::new();
    for (i, c) in s.to_owned().chars().enumerate() {
        let mut str = s.to_owned();
        str.remove(i);
        let other_permutations = permutations(&str);
        mutations.extend(
            other_permutations
                .into_iter()
                .map(|p| format!("{}{}", c, p)),
        );
    }
    mutations.into_iter().collect()
}

#[cfg(test)]
mod example_tests {
    use super::permutations;

    fn assert_ordered_equals(actual: &[String], expected: &[String]) {
        let mut actual: Vec<_> = actual.to_vec();
        let mut expected: Vec<_> = expected.to_vec();
        actual.sort_unstable();
        expected.sort_unstable();

        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right)"
        );
    }

    #[test]
    fn sample_tests() {
        let expected = vec!["a".to_string()];
        let actual = permutations("a");
        assert_ordered_equals(&actual, &expected);

        let expected = vec!["ab".to_string(), "ba".to_string()];
        let actual = permutations("ab");
        assert_ordered_equals(&actual, &expected);

        let expected = vec![
            "aabb".to_string(),
            "abab".to_string(),
            "abba".to_string(),
            "baab".to_string(),
            "baba".to_string(),
            "bbaa".to_string(),
        ];
        let actual = permutations("aabb");
        assert_ordered_equals(&actual, &expected);
    }
}
