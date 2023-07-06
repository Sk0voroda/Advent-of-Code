pub fn max_calories() -> i32 {
    let calories_data =
        std::fs::read_to_string("inputs/01_puzzle_input.txt").expect("puzzle file is missing");

    let calories = calories_data.split("\n\n").collect::<Vec<&str>>();
    let max_calories = calories
        .iter()
        .map(|item| {
            item.split("\n")
                .map(|cal| cal.parse().unwrap_or(0))
                .sum::<i32>()
        })
        .max();

    max_calories.unwrap()
}
