use std::collections::HashSet;

fn printer_error(s: &str) -> String {
    // Your code here
    let total = s.len();
    let mut errors = 0;
    let valid_letters: HashSet<char> = ('a'..='m').collect();
    for c in s.chars() {
        if valid_letters.contains(&c) {
            continue;
        } else {
            errors += 1;
        }
    }
    format!("{}/{}", errors, total)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_pass_all_the_tests_provided() {
        assert_eq!(
            &printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"),
            "3/56"
        );
        assert_eq!(
            &printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"),
            "6/60"
        );
        assert_eq!(
            &printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu"),
            "11/65"
        );
    }
}
