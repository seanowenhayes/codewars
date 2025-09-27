// https://www.codewars.com/kata/55466989aeecab5aac00003e/train/rust
use std::cmp;

struct Quadrilateral {
    width: i32,
    length: i32,
}

impl Quadrilateral {
    fn is_square(&self) -> bool {
        self.length == self.width
    }

    fn largest_square(&self) -> Self {
        let smallest_side = cmp::min(self.length, self.width);
        Self {
            width: smallest_side,
            length: smallest_side,
        }
    }

    fn remove_largest_square(&self) -> Self {
        let smallest_side = cmp::min(self.length, self.width);
        let largest_side = cmp::max(self.length, self.width);
        Self {
            width: smallest_side,
            length: largest_side - smallest_side,
        }
    }
}

fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    // your code
    if lng == wdth {
        return None;
    }
    let mut squares = Vec::new();
    let mut my_quadralateral = Quadrilateral {
        length: lng,
        width: wdth,
    };

    while !my_quadralateral.is_square() {
        squares.push(my_quadralateral.largest_square().length);
        my_quadralateral = my_quadralateral.remove_largest_square()
    }
    // we didn't add the last one as it was square so just take it's length
    squares.push(my_quadralateral.length);

    Some(squares)
}

fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
    assert_eq!(sq_in_rect(lng, wdth), exp)
}

#[test]
fn tests_sq_in_rect() {
    testing(5, 3, Some(vec![3, 2, 1, 1]));
    testing(3, 5, Some(vec![3, 2, 1, 1]));
    testing(5, 5, None);
}
