use std::fs::read_to_string;
use std::collections::HashMap;


fn is_hand_valid<'a>(hand_string: &'a str, max_needed_colors: &mut HashMap::<&'a str, u32>) -> bool { // Why did I have to use a named lifetime here?
    let mut total_colors = HashMap::<&str, u32>::new();
    total_colors.insert("red", 12);
    total_colors.insert("green", 13);
    total_colors.insert("blue", 14);

    let mut illegal_hand = false;
    let numbers_colors = hand_string.split(", ");
    for number_color in numbers_colors {
        let n_color: u32;
        let tuple_number_color = number_color.split_once(" ").unwrap();
        n_color = tuple_number_color.0.parse().unwrap();

        // PART 1
        if total_colors.get(tuple_number_color.1).unwrap() < &n_color {
            illegal_hand = true;
        }

        // PART 2
        if max_needed_colors.get(tuple_number_color.1).unwrap() < &n_color {
            max_needed_colors.insert(tuple_number_color.1, n_color);
        }
    }
    !illegal_hand
}

fn is_game_valid(game_string: &String) -> (bool, u32) {
    let mut max_needed_colors = HashMap::<&str, u32>::new();
    max_needed_colors.insert("red", 0);
    max_needed_colors.insert("green", 0);
    max_needed_colors.insert("blue", 0);
   
    let mut illegal_games = false;
    let (_game_id, hands_string) = game_string.split_once(": ").unwrap();
    let hands_itr = hands_string.split("; ");
    for hand in hands_itr {
        if !is_hand_valid(hand, &mut max_needed_colors) {
            illegal_games = true;
        }
    }
    (!illegal_games, 
    (max_needed_colors.get("red").unwrap()*
    max_needed_colors.get("green").unwrap()*
    max_needed_colors.get("blue").unwrap()))
}

pub fn solve() -> (u32, u32) {
    let games: Vec<String> = read_to_string("inputs/p2_input.txt").unwrap().lines().map(String::from).collect();

    let mut sum_of_ids: u32 = 0;
    let mut sum_of_powers: u32 = 0;

    for (game_id, game) in games.iter().enumerate() {
        let (elf_is_not_cheating, power_of_minimum_set): (bool, u32)  = is_game_valid(game);
        if elf_is_not_cheating {
            sum_of_ids += (game_id + 1) as u32;
        }
        sum_of_powers += power_of_minimum_set;
    }
    (sum_of_ids, sum_of_powers)
}

