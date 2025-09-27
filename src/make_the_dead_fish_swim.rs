fn parse(code: &str) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut current_value: i32 = 0;
    for c in code.chars() {
        match c {
            'i' => current_value += 1,
            'd' => current_value -= 1,
            's' => current_value = current_value * current_value,
            'o' => {
                result.push(current_value);
            }
            _ => (),
        }
    }

    result
}

#[cfg(test)]
mod example_programs {
    use super::parse;

    #[test]
    fn example_iiisdoso() {
        assert_eq!(parse("iiisdoso"), vec![8, 64]);
    }

    #[test]
    fn example_iiisxxxdoso() {
        assert_eq!(parse("iiisxxxdoso"), vec![8, 64]);
    }
}
