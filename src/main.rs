pub mod p1;
pub mod p2;
pub mod p3;
pub mod p4;

fn main() {
    println!("P1: { }", p1::solve());
    let (p2_part1, p2_part2) = p2::solve();
    println!("P2 part1: { }, part2: { }", p2_part1, p2_part2);
    println!("P3: { }", p3::solve());
    println!("P4: { }", p4::solve());
}
