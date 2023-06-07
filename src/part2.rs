use std::collections::HashMap;

// AX -> rock-rock -> draw
// AY -> rock-paper -> win
// AZ -> rock-scissors -> loss
// BX -> paper-rock -> loss
// BY -> paper-paper -> draw
// BZ -> paper-scissors -> win
// CX -> scissors-rock -> win
// CY -> scissors-paper -> loss
// CZ -> scissors-scissors -> draw

pub fn get_part2_answer(input: String) -> i32 {
    let game_scores = HashMap::from([
        ("AX", 'C'),
        ("AY", 'A'),
        ("AZ", 'B'),
        ("BX", 'A'),
        ("BY", 'B'),
        ("BZ", 'C'),
        ("CX", 'B'),
        ("CY", 'C'),
        ("CZ", 'A'),
    ]);

    let total_points: i32 = input
        .lines()
        .map(|x| {
            let round: String = x.split_whitespace().collect();
            let shape_score = match round.chars().nth(1) {
                Some('X') => 0,
                Some('Y') => 3,
                Some('Z') => 6,
                None => 0,
                _ => 0,
            };
            let round_score = match game_scores.get(round.as_str()) {
                Some('A') => 1,
                Some('B') => 2,
                Some('C') => 3,
                None => 0,
                _ => 0,
            };

            round_score + shape_score
        })
        .sum();
    println!("total points part2: {:?}", total_points);
    return total_points;
}
