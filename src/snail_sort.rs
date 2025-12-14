use std::collections::VecDeque;

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix.len() == 0 {
        Vec::new()
    } else if matrix.len() == 1 {
        matrix[0].to_owned()
    } else {
        let mut unsorted: VecDeque<Vec<i32>> = matrix.iter().map(|a| a.clone()).collect();
        // top
        let mut sorted: Vec<i32> = unsorted.pop_front().unwrap();
        // right
        let mut unsorted: VecDeque<Vec<i32>> = unsorted
            .into_iter()
            .map(|mut row| {
                sorted.push(row.pop().unwrap());
                row
            })
            .collect();
        // bottom
        let mut bottom = unsorted.pop_back().unwrap();
        bottom.reverse();
        sorted.append(&mut bottom);
        // left -
        let mut unsorted: VecDeque<Vec<i32>> = unsorted
            .into_iter()
            // maybe there is no left at this point!
            .filter(|row| !row.is_empty())
            .rev()
            .map(|mut row| {
                // only get here if there is a left
                sorted.push(row.remove(0));
                row
            })
            .collect();
        let next_iteration_unsorted = unsorted.make_contiguous();
        next_iteration_unsorted.reverse();
        sorted.append(&mut snail(next_iteration_unsorted));
        sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test2() {
        let square = &[vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }

    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test5() {
        let square = &[
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        let expected = vec![
            1, 2, 3, 4, 5, 10, 15, 20, 25, 24, 23, 22, 21, 16, 11, 6, 7, 8, 9, 14, 19, 18, 17, 12,
            13,
        ];
        assert_eq!(snail(square), expected);
    }
    // tests::random_tests
    // assertion failed: `(left == right)`
    //   left: `[620, 655, 572, 977, 198, 331, 498, 918, 310, 728, 485, 1000, 603, 823, 576, 249]`,
    //  right: `[620, 655, 572, 977, 198, 331, 498, 918, 310, 728, 1000, 485, 603, 823, 576, 249]`:
    // Failed with input [[620, 655, 572, 977], [485, 603, 823, 198], [1000, 249, 576, 331], [728, 310, 918, 498]]
    //
    // #[test]
    // fn fixed_tests() {
    //     let
    // }
}
