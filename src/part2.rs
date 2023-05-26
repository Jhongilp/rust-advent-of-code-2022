pub fn get_part2_answer(input: String) -> i32 {
    let mut result = 0;
    input.lines().for_each(|pairs| {
        let pairs_arr: Vec<&str> = pairs.split(",").collect();
        let first_pair_start = pairs_arr[0].split("-").collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap();
        let first_pair_end = pairs_arr[0].split("-").collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();
        let second_pair_start = pairs_arr[1].split("-").collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap();
        let second_pair_end = pairs_arr[1].split("-").collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();

        let range_overlap = if first_pair_start <= second_pair_start {
            first_pair_end >= second_pair_start
        } else {
            second_pair_end >= first_pair_start
        };

        if range_overlap {
            result += 1;
        }
    });
    println!("result: {}", result);
    result
}
