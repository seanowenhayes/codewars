fn reduce_fraction(fraction: (u32, u32)) -> (u32, u32) {
    let (numerator, denominator) = fraction;
    let mut largest_common_denominator = 1;
    let smallest = if denominator < numerator {
        denominator
    } else {
        numerator
    };
    for n in 2..smallest {
        let is_num_div_by_n = numerator % n == 0;
        let is_den_div_by_n = denominator % n == 0;
        if is_num_div_by_n && is_den_div_by_n {
            largest_common_denominator = n;
        }
    }

    (
        numerator / largest_common_denominator,
        denominator / largest_common_denominator,
    )
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::reduce_fraction;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(f: (u32, u32), expected: (u32, u32)) {
        assert_eq!(
            reduce_fraction(f),
            expected,
            "{ERR_MSG} with fraction = {} / {}",
            f.0,
            f.1
        )
    }

    #[test]
    fn simple_fractions() {
        dotest((60, 20), (3, 1));
        dotest((80, 120), (2, 3));
        dotest((4, 2), (2, 1));
        dotest((45, 120), (3, 8));
        dotest((1000, 1), (1000, 1));
        dotest((1, 1), (1, 1));
    }
}
