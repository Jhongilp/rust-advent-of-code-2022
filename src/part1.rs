pub fn get_part1_answer(input: String) -> i32 {
    let mut sum_of_priorities = 0;
    input.lines().for_each(|rucksack| {
        let compartment1 = rucksack[0..rucksack.len() / 2]
            .chars()
            .collect::<Vec<char>>();
        let compartment2 = rucksack[rucksack.len() / 2..rucksack.len()]
            .chars()
            .collect::<Vec<char>>();
        let item_in_both = compartment1
            .iter()
            .filter(|&item| compartment2.contains(item))
            .collect::<Vec<&char>>();

        let offset = if item_in_both[0].is_ascii_uppercase() {
            64 - 26 // 64 is the ascii code for A, and we want to start at 27
        } else {
            96
        };
        sum_of_priorities += item_in_both[0].clone() as u32 - offset;
    });
    println!("sum_of_priorities: {}", sum_of_priorities);
    sum_of_priorities as i32
}
