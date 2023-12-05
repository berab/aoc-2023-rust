use std::fs::read_to_string;

pub fn solve() -> (u32, u32) {
    //let cards: Vec<String> = read_to_string("inputs/p5_input.txt");//.unwrap().lines().map(String::from).collect();
    let cards: Vec<String> = read_to_string("inputs/p5_input.txt").unwrap().split("\n\n").map(String::from).collect();//.unwrap().lines().map(String::from).collect();
    //println!("{:?}", cards);
    (3, 3)
}
