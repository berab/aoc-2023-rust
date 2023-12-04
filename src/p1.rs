use std::fs::read_to_string;

pub fn solve() -> u32 {
    let lines: Vec<String> = read_to_string("inputs/p1_input.txt").unwrap().lines().map(String::from).collect();

    let mut sum: u32 = 0;
    for line in lines {
        let mut first_digit: u32 = 0;
        let mut second_digit: u32 = 0;

        for letter in line.chars() {
            match letter.to_digit(10) {
                Some(digit) => {
                    if first_digit == 0 {
                        (first_digit, second_digit) = (digit, digit);
                    } else {
                        second_digit = digit;
                    }
                }
                None => { }
            }
        }
        sum += first_digit*10 + second_digit;
    }
    sum
}
