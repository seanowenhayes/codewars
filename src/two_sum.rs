use std::u32;

fn sum_two_smallest_numbers(numbers: &[u32]) -> u32 {
    let nums = Vec::from(numbers);
    let mut smallest = u32::MAX;
    let mut second_smallest = u32::MAX;
    for &num in numbers {
        if num < smallest {
            second_smallest = smallest;
            smallest = num;
        } else if num < second_smallest || second_smallest == 0 {
            second_smallest = num;
        }
    }
    smallest + second_smallest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(
            sum_two_smallest_numbers(&[5, 8, 12, 19, 22]),
            13,
            "Incorrect result for [5, 8, 12, 19, 22]"
        );
        assert_eq!(
            sum_two_smallest_numbers(&[15, 28, 4, 2, 43]),
            6,
            "Incorrect result for [15, 28, 4, 2, 43]"
        );
        assert_eq!(
            sum_two_smallest_numbers(&[23, 71, 33, 82, 1]),
            24,
            "Incorrect result for [23, 71, 33, 82, 1]"
        );
        assert_eq!(
            sum_two_smallest_numbers(&[52, 76, 14, 12, 4]),
            16,
            "Incorrect result for [52, 76, 14, 12, 4]"
        );
        assert_eq!(
            sum_two_smallest_numbers(&[1, 1, 5, 5]),
            2,
            "Incorrect result for [1, 1, 5, 5]"
        );
    }
}
