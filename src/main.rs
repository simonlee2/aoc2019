mod day1;
mod day2;

fn main() {
    println!("Day 1: ({}, {})", day1::solve().0, day1::solve().1);
    println!("Day 2: {:?}", day2::solve());
}
