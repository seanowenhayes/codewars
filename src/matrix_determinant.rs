fn determinant(matrix: &[Vec<i64>]) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::determinant;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[Vec<i64>], expected: i64) {
        assert_eq!(determinant(a), expected, "{ERR_MSG}")
    }

    #[test]
    fn sample_tests() {
        dotest(&[vec![1]], 1);
        dotest(&[vec![1, 3], vec![2, 5]], -1);
        dotest(&[vec![2, 5, 3], vec![1, -2, -1], vec![1, 3, 4]], -20);
    }
}
