pub fn get_part1_answer(input: String) -> i32 {
    input.lines().for_each(|pairs| {
        let pairs_arr: Vec<&str> = pairs.split(",").collect();
        let first_pair_start = pairs_arr[0].split("-").collect::<Vec<&str>>()[0];
        let first_pair_end = pairs_arr[0].split("-").collect::<Vec<&str>>()[1];
        let second_pair_start = pairs_arr[1].split("-").collect::<Vec<&str>>()[0];
        let second_pair_end = pairs_arr[1].split("-").collect::<Vec<&str>>()[1];
        
    });
    0

}
