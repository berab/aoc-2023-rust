use std::fs::read_to_string;

pub fn solve() -> u32 {
    let cards: Vec<String> = read_to_string("inputs/p4_input.txt").unwrap().lines().map(String::from).collect();

    let mut n_won: u32 = 0;
    for card in cards {
        let (winning_numbers, my_numbers) = card.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winning_vector: Vec<&str> = winning_numbers.split(" ").map(|x| x.trim()).collect();//.parse().unwrap()).collect();
        //println!("{:?}", winning_numbers);
        println!("{:?}", winning_vector);
        //let my_vector: Vec<u32> = my_numbers.split(" ").map(|x| x.trim().parse().unwrap()).collect();
        //for number in my_vector {
        //    if winning_vector.contains(&number) { n_won += 1 }
        //}
    }
    //u32::pow(2, n_won-1)
    3
}
