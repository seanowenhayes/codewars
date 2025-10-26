use std::cmp::Ordering;

fn order_weight(s: &str) -> String {
    let mut weights: Vec<(u32, String)> = Vec::new();
    s.split(" ").for_each(|weight| {
        let sum = weight
            .chars()
            .map(|c| c.to_digit(10).expect("Found a non digit"))
            .sum();
        weights.push((sum, weight.to_owned()));
    });

    weights.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });
    let l: Vec<String> = weights
        .iter()
        .map(|(_, weight)| weight.to_owned())
        .collect();
    l.join(" ")
}

fn testing(s: &str, exp: &str) -> () {
    assert_eq!(order_weight(s), exp)
}

#[test]
fn basics_order_weight() {
    testing("103 123 4444 99 2000", "2000 103 123 4444 99");
    testing(
        "2000 10003 1234000 44444444 9999 11 11 22 123",
        "11 11 2000 10003 22 123 1234000 44444444 9999",
    );
}
