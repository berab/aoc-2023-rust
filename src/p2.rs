use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

struct Colors {
    red: u32,
    green: u32,
    blue: u32,
}

fn is_hand_valid(hand_string: &str) -> bool {
    let max_colors = Colors { red: 12, green: 13, blue: 14 };

    let numbers_colors = hand_string.split(", ");
    for number_color in numbers_colors {

        // Why does this have to be mutable?
        let mut num_color_iterator = number_color.split_whitespace(); 

        let num_of_color: u32;
        match num_color_iterator.next() {
            Some(num) => {
                match num.parse() {
                    Ok(n) => {
                        num_of_color = n;
                    }
                    Err(e) => {
                        println!("{ }", e);
                        return false; // Q:Are these necessary?
                    }
                }
            }
            None => {
                return false;
                }
        }
        

        match num_color_iterator.next() {
            Some(color) => {
                //println!("for { } { }", num_of_color, color);
                if color == "red" && num_of_color > max_colors.red {
                    return false
                } else if color == "green" && num_of_color > max_colors.green {
                    return false
                } else if color == "blue" && num_of_color > max_colors.blue {
                    return false
                }
            }
            None => { 
                return false; 
                }
        }
    }
    return true;

}

fn is_game_valid(game_string: &String) -> bool {
    
    let mut split_gameid_trials = game_string.split(": "); // I had to make this mut. not sure why?
    match split_gameid_trials.nth(1) {
        Some(hands) => {
            let hand_iteriator = hands.split("; ");
            for hand in hand_iteriator {
                if !is_hand_valid(hand) {
                    return is_hand_valid(hand);
                }
            }
        }
        None => { 
            println!("Smt unexpected happening over here!");
            return false
        }
    }
    true
}

pub fn solve_part1() -> u32 {
    let games: Vec<String> = read_lines("inputs/p2_input.txt");

    let mut sum_of_ids: u32 = 0;
    for (game_id, game) in games.iter().enumerate() {
        let elf_is_not_cheating: bool = is_game_valid(game);
        if elf_is_not_cheating {
            sum_of_ids += (game_id + 1) as u32;
        }
    }
    sum_of_ids
}

