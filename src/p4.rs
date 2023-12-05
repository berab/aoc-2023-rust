use std::fs::read_to_string;

pub fn solve() -> (u32, u32) {
    let cards: Vec<String> = read_to_string("inputs/p4_input.txt").unwrap().lines().map(String::from).collect();
    //let cards: Vec<String> = read_to_string("inputs/p4_small.txt").unwrap().lines().map(String::from).collect();
    let mut won_cards: Vec<u32> = vec![1; cards.len()]; // 1 each at the initially.

    let mut total_won: u32 = 0;
    for (card_idx, card) in cards.iter().enumerate() {
        let mut n_won: u32 = 0;
        let (winners, numbers) = card.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winner_vec_str: Vec<&str> = winners.split(" ").map(|x| x.trim()).collect();
        let number_vec_str: Vec<&str> = numbers.split(" ").map(|x| x.trim()).collect();
        let mut winner_vec: Vec<u32> = Vec::new();

        for winner_str in &winner_vec_str {
            match winner_str.parse() {
                Ok(winner) => { winner_vec.push(winner) }
                _ => { }
            }
        }

        for number_str in &number_vec_str {
            match number_str.parse() {
                Ok(number) => { 
                    if winner_vec.contains(&number) { n_won += 1 }
                }
                _ => { }
            }
        }

        // part 1
        if n_won > 0 { total_won += u32::pow(2, n_won-1) }
        // part 2
        for _ in 0..(won_cards[card_idx]) {
            for j in 1..(n_won+1) { won_cards[card_idx + j as usize] += 1 }
        }
    }
    (total_won, won_cards.iter().sum())
}
