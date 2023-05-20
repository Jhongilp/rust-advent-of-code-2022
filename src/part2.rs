pub fn get_part2_answer(input: String) -> i32 {
    let mut sum_of_priorities = 0;
    let rucksacks = input.lines().collect::<Vec<&str>>();
    for (i, rucksack) in rucksacks.iter().enumerate() {
        if i % 3 == 0 {
            let r1 = rucksack;
            let r2 = rucksacks[i + 1];
            let r3 = rucksacks[i + 2];
            let common_badget = r1
                .chars()
                .filter(|c| r2.contains(*c) && r3.contains(*c))
                .collect::<Vec<char>>();
            // println!("common_badget: {:?}", common_badget[0]);
            let offset = if common_badget[0].is_ascii_uppercase() {
                64 - 26 // 64 is the ascii code for A, and we want to start at 27
            } else {
                96
            };
            sum_of_priorities += common_badget[0].clone() as u32 - offset;
        }
    }
    println!("sum_of_priorities: {}", sum_of_priorities);
    sum_of_priorities as i32
}
