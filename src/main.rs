// use aoc_1::part1::get_part1_answer;
use aoc_1::part2::get_part2_answer;
use std::fs;

fn main() {
    println!("Hello, world!");
    let input = fs::read_to_string("input.txt").expect("Filed should be read");
    // get_part1_answer(input);
    get_part2_answer(input);
}
