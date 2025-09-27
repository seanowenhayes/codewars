/// Converts a number to a string representating roman numeral.
fn num_as_roman(num: i32) -> String {
    let mut conversion = String::new();
    let mut num = num;
    while num > 0 {
        if num >= 1000 {
            conversion.push('M');
            num -= 1000;
        } else if num >= 900 {
            conversion.push('C');
            conversion.push('M');
            num -= 900;
        } else if num >= 500 {
            conversion.push('D');
            num -= 500;
        } else if num >= 400 {
            conversion.push('C');
            conversion.push('D');
            num -= 400;
        } else if num >= 100 {
            conversion.push('C');
            num -= 100;
        } else if num >= 90 {
            conversion.push('X');
            conversion.push('C');
            num -= 90;
        } else if num >= 50 {
            conversion.push('L');
            num -= 50;
        } else if num >= 40 {
            conversion.push('X');
            conversion.push('L');
            num -= 40;
        } else if num >= 10 {
            conversion.push('X');
            num -= 10;
        } else if num == 9 {
            conversion.push('I');
            conversion.push('X');
            num -= 9;
        } else if num >= 5 {
            conversion.push('V');
            num -= 5;
        } else if num == 4 {
            conversion.push('I');
            conversion.push('V');
            num -= 4;
        } else if num >= 1 {
            conversion.push('I');
            num -= 1;
        }
    }
    conversion
}

#[test]
fn returns_expected() {
    assert_eq!(num_as_roman(1), "I");
    assert_eq!(num_as_roman(2), "II");
    assert_eq!(num_as_roman(3), "III");
    assert_eq!(num_as_roman(4), "IV");
    assert_eq!(num_as_roman(5), "V");
    assert_eq!(num_as_roman(6), "VI");
    assert_eq!(num_as_roman(7), "VII");
    assert_eq!(num_as_roman(8), "VIII");
    assert_eq!(num_as_roman(9), "IX");
    assert_eq!(num_as_roman(10), "X");
    assert_eq!(num_as_roman(11), "XI");
    assert_eq!(num_as_roman(14), "XIV");
    assert_eq!(num_as_roman(15), "XV");
    assert_eq!(num_as_roman(40), "XL");
    assert_eq!(num_as_roman(90), "XC");
    assert_eq!(num_as_roman(140), "CXL");
    assert_eq!(num_as_roman(182), "CLXXXII");
    assert_eq!(num_as_roman(400), "CD");
    assert_eq!(num_as_roman(900), "CM");
    assert_eq!(num_as_roman(1400), "MCD");
    assert_eq!(num_as_roman(1990), "MCMXC");
    assert_eq!(num_as_roman(1875), "MDCCCLXXV");
}
