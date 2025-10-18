fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i64) {
        if n % i == 0 {
            println!("Not a primer {}", n);
            return false;
        } else {
            println!("Dunno {}", n);
        }
    }
    true
}

fn find_factors(n: i64) -> (i64, i64) {
    for i in 2..=((n as f64).sqrt() as i64) {
        if n % i == 0 {
            println!("i is {} and n / i is {}", i, n / i);
            return (i, n / i);
        }
    }
    println!("Should't get here n is {}", n);
    (1, 2)
}

fn prime_factors(n: i64) -> String {
    // your code
    if is_prime(n) {
        format!("{}", n)
    } else {
        let (first, second) = find_factors(n);
        format!("({}**{})", prime_factors(first), prime_factors(second))
    }
}

fn testing(n: i64, exp: &str) -> () {
    assert_eq!(&prime_factors(n), exp)
}

#[test]
fn basics_prime_factors() {
    // testing(2, "(2)");
    // testing(7, "(7)");
    testing(4, "(2**2)");
    testing(27, "(3**3)");
    // testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    // testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
}
