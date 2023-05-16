use std::fs;

fn main() {
    println!("Hello, world!");
    let input = fs::read_to_string("input.txt").expect("Filed should be read");
    println!("Input data \n{}", input);
    let mut total_calories_by_elf: Vec<i32> = Vec::new();
    let mut max = 0;
    let mut sum = 0;
    input.lines().for_each(|item| {
        let calories = item.parse::<i32>().unwrap_or_else(|_| {
            // println!("No a number, then it must start a new group of calories");
            if sum > max {
                max = sum;
            }
            total_calories_by_elf.push(sum);
            sum = 0;
            0
        });
        sum += calories;
        // println!("Calories {}", calories);
        // println!("Calories sum  {}", sum);
    });

    total_calories_by_elf.sort();
    let mut total_largest_three = 0;
    let last_three =
        total_calories_by_elf.get(total_calories_by_elf.len() - 3..total_calories_by_elf.len());

    for i in last_three {
        for j in 0..3 {
            if let Some(x) = Some(i.get(j)).unwrap() {
                println!("From last three Some: {:?}", x);
                total_largest_three += x;
            }
        }
    }
    println!("The largest calories group is {}", max);

    println!(
        "The three largest calories group are {}",
        total_largest_three
    );
}
