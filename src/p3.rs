use std::fs::read_to_string;


// Checks if any adjacents is a symbol (including itself)
fn is_adjacent(rows: &Vec<String>, row_idx: usize, col_idx: usize) -> bool {
    let symbols = 
        [
     '#',
     '$',
     '%',
     '&',
     '*',
     '+',
     '-',
     '/',
     '=',
     '@'];
    let rows_len = rows.len() as isize;
    let cols_len = rows[0].len() as isize;

    // Checking all adjacents (3x3) including itself
    for i in 0..3 {
        for j in 0..3 {
            let symb_row_idx = row_idx as isize + i - 1;
            let symb_col_idx = col_idx as isize + j -1;
            if symb_row_idx >= 0 && symb_col_idx >= 0 && symb_row_idx < rows_len && symb_col_idx < cols_len {
                let symbol = rows[symb_row_idx as usize].chars().nth(symb_col_idx as usize).unwrap();
                if symbols.contains(&symbol) {
                    return true
                }
            }
        }
    }
    false
}

fn sum_numbers(digits: &Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    let mut i = (digits.len() - 1 ) as u32;
    for digit in digits {
        sum += digit*u32::pow(10, i);
        if i > 0 { i -= 1 };
    }
    sum
}

fn check_sum_flush(symbolled: bool, mut sum: u32, cur_digits: Vec<u32>) -> (bool, u32, Vec<u32>) {
    if symbolled { sum += sum_numbers(&cur_digits) }; // Check. Sum
    (false, sum, Vec::<u32>::new())
}


pub fn solve() -> u32 {
    let rows: Vec<String> = read_to_string("inputs/p3_input.txt").unwrap().lines().map(String::from).collect();
    //let rows: Vec<String> = read_to_string("inputs/p3_edge.txt").unwrap().lines().map(String::from).collect();
    //let rows: Vec<String> = read_to_string("inputs/p3_small.txt").unwrap().lines().map(String::from).collect();
    let rows_len = rows.len();
    let cols_len = rows[0].len();
    let mut sum: u32 = 0;

    for row_idx in 0..rows_len {

        let mut cur_digits: Vec<u32> = Vec::new();
        let mut symbolled: bool = false;
        for col_idx in 0..cols_len {
            match rows[row_idx].chars().nth(col_idx).unwrap().to_digit(10) {
                Some(digit) => {
                    cur_digits.push(digit);
                    if !symbolled && is_adjacent(&rows, row_idx, col_idx) { symbolled = true };
                }
                None => { // dot/symbol
                    (symbolled, sum, cur_digits) = check_sum_flush(symbolled, sum, cur_digits);
                } 
            }
            if col_idx == cols_len -1 { 
                (symbolled, sum, cur_digits) = check_sum_flush(symbolled, sum, cur_digits);
            }
        }
    }
    sum
}

