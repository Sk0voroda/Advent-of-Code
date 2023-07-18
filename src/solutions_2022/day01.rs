// https://adventofcode.com/2022/day/1

use itertools::Itertools;

// part one
pub fn top_calories() -> usize {
    let file_data =
        std::fs::read_to_string("inputs/2022/01.txt").expect("puzzle for day 1 file is missing");

    file_data
        .split("\n\n")
        .map(|item| {
            item.split("\n")
                .map(|cal| cal.parse::<usize>().unwrap_or(0))
                .sum()
        })
        .max()
        .unwrap()
}

// part two
pub fn top_three_calories() -> usize {
    let file_data =
        std::fs::read_to_string("inputs/2022/01.txt").expect("puzzle for day 1 file is missing");

    let mut top_three = file_data
        .split("\n\n")
        .map(|item| {
            item.split("\n")
                .map(|cal| cal.parse::<usize>().unwrap_or(0))
                .sum()
        })
        .collect::<Vec<usize>>();

    top_three.sort();
    top_three.reverse();

    top_three.iter().take(3).copied().sum()
}

// part two using Itertools
pub fn top_three_calories_itertools() -> usize {
    let file_data =
        std::fs::read_to_string("inputs/2022/01.txt").expect("puzzle for day 1 file is missing");

    file_data
        .split("\n\n")
        .map(|item| {
            item.split('\n')
                .map(|cal| cal.parse().unwrap_or(0))
                .sum::<usize>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum()
}
