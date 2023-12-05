pub mod p1;
pub mod p2;
pub mod p3;
pub mod p4;
pub mod p5;

fn main() {
    println!("P1: { }", p1::solve()); // diddnt know part2 exists
    let (p2_part1, p2_part2) = p2::solve(); 
    println!("P2 part1: { }, part2: { }", p2_part1, p2_part2);
    println!("P3: { }", p3::solve()); // P2 seemed so diff
    let (p4_part1, p4_part2) = p4::solve();
    println!("P4 part1: { }, part2: { }", p4_part1, p4_part2);
    let (p5_part1, p5_part2) = p5::solve();
    println!("P5 part1: { }, part2: { }", p5_part1, p5_part2);
}
