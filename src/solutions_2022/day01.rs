// https://adventofcode.com/2022/day/1

// part one
pub fn top_calories() -> usize {
    let file_data =
        std::fs::read_to_string("inputs/2022/01.txt").expect("puzzle day 1 file is missing");

    let max_calories = file_data
        .split("\n\n")
        .map(|item| {
            item.split("\n")
                .map(|cal| cal.parse::<usize>().unwrap_or(0))
                .sum()
        })
        .max();

    max_calories.unwrap()
}

// part two
pub fn top_three_calories() -> usize {
    let file_data =
        std::fs::read_to_string("inputs/2022/01.txt").expect("puzzle day 1 file is missing");

    let mut top_three = file_data
        .split("\n\n")
        .map(|item| {
            item.lines()
                .map(|cal| cal.parse::<usize>().unwrap_or(0))
                .sum()
        })
        .collect::<Vec<usize>>();

    top_three.sort();
    top_three.reverse();

    top_three.iter().take(3).copied().sum()
}
