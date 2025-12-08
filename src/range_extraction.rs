mod solution {
    enum Item {
        Range(i32, i32),
        Num(i32),
        None,
    }
    impl Item {
        fn to_string(&self) -> String {
            match self {
                Self::None => "".to_owned(),
                Self::Num(num) => num.to_string(),
                Self::Range(low, high) => {
                    if (low + 1) == *high {
                        format!("{},{}", low, high)
                    } else {
                        format!("{}-{}", low, high)
                    }
                }
            }
        }
    }
    pub fn range_extraction(a: &[i32]) -> String {
        let mut items: Vec<Item> = Vec::new();
        let mut current = Item::None;
        for num in a {
            match current {
                Item::None => current = Item::Num(*num),
                Item::Num(current_num) => {
                    if *num == (current_num + 1) {
                        current = Item::Range(current_num, *num);
                    } else {
                        items.push(current);
                        current = Item::Num(*num);
                    }
                }
                Item::Range(low, high) => {
                    if *num == (high + 1) {
                        current = Item::Range(low, *num)
                    } else {
                        items.push(current);
                        current = Item::Num(*num)
                    }
                }
            }
        }
        items.push(current);
        let formatted_items: Vec<String> = items.iter().map(|item| item.to_string()).collect();
        formatted_items.join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ]),
            "-6,-3-1,3-5,7-11,14,15,17-20"
        );
        assert_eq!(
            solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20]),
            "-3--1,2,10,15,16,18-20"
        );
    }
}
