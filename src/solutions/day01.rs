// Finds Elf with max calories carring

pub fn max_calories() -> i32 {
    let calories_data = std::fs::read_to_string("inputs/01.txt").expect("puzzle file is missing");

    let max_calories = calories_data
        .split("\n\n")
        .map(|item| {
            item.split("\n")
                .map(|cal| cal.parse::<i32>().unwrap_or(0))
                .sum()
        })
        .max();

    max_calories.unwrap()
}
