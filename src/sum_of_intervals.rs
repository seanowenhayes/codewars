use std::cmp::max;

fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
    let mut sorted_intervals = Vec::from(intervals);
    sorted_intervals.sort_by(|(first_a, _), (first_b, _)| first_a.cmp(first_b));
    let mut merged_intervals: Vec<(i32, i32)> = Vec::new();
    for interval in sorted_intervals {
        if let Some(last) = merged_intervals.pop() {
            let (last_low, last_high) = last;
            let (current_low, current_high) = interval;
            if current_low < last_high {
                merged_intervals.push((last_low, max(current_high, last_high)));
            } else {
                merged_intervals.push((last_low, last_high));
                merged_intervals.push((current_low, current_high));
            }
        } else {
            merged_intervals.push(interval);
        }
    }
    merged_intervals.iter().map(|(low, high)| high - low).sum()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod sample_tests {
    use super::*;
    const ERR_MSG: &str = "\nYour result (left) did not match expected output (right).";

    #[test]
    fn non_overlapping_intervals() {
        assert_eq!(sum_intervals(&[(1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(1, 5), (6, 10)]), 8, "{}", ERR_MSG);
    }

    #[test]
    fn overlapping_intervals() {
        assert_eq!(sum_intervals(&[(1, 5), (1, 5)]), 4, "{}", ERR_MSG);
        assert_eq!(sum_intervals(&[(1, 4), (7, 10), (3, 5)]), 7, "{}", ERR_MSG);
        assert_eq!(
            sum_intervals(&[(1, 20), (2, 19), (5, 15), (8, 12)]),
            19,
            "{}",
            ERR_MSG
        );
    }

    #[test]
    fn large_intervals() {
        assert_eq!(
            sum_intervals(&[(-1_000_000_000, 1_000_000_000)]),
            2_000_000_000,
            "{}",
            ERR_MSG
        );
        assert_eq!(
            sum_intervals(&[(0, 20), (-100_000_000, 10), (30, 40)]),
            100_000_030,
            "{}",
            ERR_MSG
        );
    }
}
