// https://adventofcode.com/2022/day/3

use std::result;

use itertools::Itertools;

fn char_to_number(c: char) -> usize {
    match c {
        // TODO: read about ..= operator
        'a'..='z' => (c as usize) - ('a' as usize) + 1,
        'A'..='Z' => (c as usize) - ('A' as usize) + 27,
        _ => 0,
    }
}

// part one
pub fn rucksack_reorganization() -> usize {
    let file_data =
        std::fs::read_to_string("inputs/2022/03.txt").expect("puzzle for day 3 file is missing");

    let sum_of_priorities = file_data
        .lines()
        // both sub strings len is always same
        .map(|line| (&line[..line.len() / 2], &line[line.len() / 2..]))
        // find same chars in sub strings
        .map(|(first, second)| {
            let mut priotiry = 0;
            let mut same = String::new();

            for c in first.chars() {
                if second.contains(c) && !same.contains(c) {
                    priotiry += char_to_number(c);
                    same.push(c)
                }
            }

            priotiry
        })
        .sum::<usize>();

    sum_of_priorities
}

// part two
pub fn rucksack_reorganization_badges() -> usize {
    let file_data =
        std::fs::read_to_string("inputs/2022/03.txt").expect("puzzle for day 3 file is missing");

    let sum_of_badges_chars = file_data.split('\n').collect::<Vec<&str>>();
    let sum_of_badges_chars = sum_of_badges_chars
        .chunks(3)
        .map(|group| {
            if let Some([first, second, third]) = group.get(..3) {
                let mut same = String::new();
                let mut priotiry = 0;

                for c in first.chars() {
                    if second.contains(c) && third.contains(c) && !same.contains(c) {
                        same.push(c);
                        priotiry += char_to_number(c);
                    }
                }

                return priotiry;
            }

            0
        })
        .sum::<usize>();

    sum_of_badges_chars
}
