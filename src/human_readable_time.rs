fn pad(time: u32) -> String {
    if time < 10 {
        format!("0{}", time)
    } else {
        format!("{}", time)
    }
}

const seconds_in_minute: u32 = 60;
const seconds_in_hour: u32 = 60 * 60;

fn make_readable(seconds: u32) -> String {
    let hours = seconds / seconds_in_hour;
    let seconds_left = seconds - hours * seconds_in_hour;
    let minutes = seconds_left / seconds_in_minute;
    let seconds_left = seconds_left % seconds_in_minute;
    format!("{}:{}:{}", pad(hours), pad(minutes), pad(seconds_left))
}
#[cfg(test)]
mod tests {
    use super::make_readable;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: u32, expected: &str) {
        assert_eq!(make_readable(s), expected, "{ERR_MSG} with seconds = {s}")
    }

    #[test]
    fn fixed_tests() {
        dotest(0, "00:00:00");
        dotest(59, "00:00:59");
        dotest(60, "00:01:00");
        dotest(3599, "00:59:59");
        dotest(3600, "01:00:00");
        dotest(86399, "23:59:59");
        dotest(86400, "24:00:00");
        dotest(359999, "99:59:59");
    }
}
