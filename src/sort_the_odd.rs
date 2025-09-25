fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odds: Vec<i32> = arr.iter().filter(|&a| a % 2 != 0).cloned().collect();
    odds.sort();
    let mut result = Vec::new();
    for num in arr {
        if num % 2 == 0 {
            result.push(*num);
        } else {
            let odd = odds.remove(0);
            result.push(odd);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(sort_array(&[]), []);
    }
}
