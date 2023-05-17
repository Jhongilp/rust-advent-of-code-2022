use std::collections::HashMap;
use std::fs;

// AX -> rock-rock -> draw
// AY -> rock-paper -> win
// AZ -> rock-scissors -> loss
// BX -> paper-rock -> loss
// BY -> paper-paper -> draw
// BZ -> paper-scissors -> win
// CX -> scissors-rock -> win
// CY -> scissors-paper -> loss
// CZ -> scissors-scissors -> draw

fn main() {
    println!("Hello, world!");
    let input = fs::read_to_string("input.txt").expect("Filed should be read");
    // println!("Input data \n{}", input);
    let game_scores = HashMap::from([
        ("AX", 3),
        ("AY", 6),
        ("AZ", 0),
        ("BX", 0),
        ("BY", 3),
        ("BZ", 6),
        ("CX", 6),
        ("CY", 0),
        ("CZ", 3),
    ]);

    let total_points: i32 = input.lines().map(|x| {
        let round: String = x.split_whitespace().collect();
        let shape_score = match round.chars().nth(1) {
            Some('X') => 1,
            Some('Y') => 2,
            Some('Z') => 3,
            None => 0,
            _ => 0, 
        };
        let score = match game_scores.get(round.as_str()) {
            Some(x) => x.clone() + shape_score,
            None => 0,
        };
        
        score
    }).sum();
    println!("total points {:?}", total_points);
}
