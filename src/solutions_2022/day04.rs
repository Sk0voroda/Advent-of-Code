pub mod clean_range {
    #[derive(Debug)]
    pub struct CleanRange {
        start: usize,
        end: usize,
    }

    impl CleanRange {
        pub fn new(start: usize, end: usize) -> Self {
            if start <= end {
                return Self { start, end };
            }

            panic!("Error: wrong range! Start of range value should be smaller or equal to the end of range!")
        }
        // pub fn new(as_string: &str) -> Self {}
        pub fn contains(&self, other: &Self) -> bool {
            self.start <= other.start && other.end <= self.end
        }
        pub fn contained(first: &CleanRange, second: &CleanRange) -> bool {
            first.contains(&second) || second.contains(&first)
        }
    }
}

use clean_range::*;

pub fn camp_cleanup() -> usize {
    let file_data =
        std::fs::read_to_string("inputs/2022/04.txt").expect("puzzle for day 4 file is missing");

    // TODO: rewrite using tuple not vector i guess
    let num_of_contained_ranges = file_data
        .lines()
        .map(|line| {
            line.split(',')
                .map(|ranges| {
                    let range_values = ranges
                        .split('-')
                        .map(|val| val.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();

                    CleanRange::new(range_values[0], range_values[1])
                })
                .collect::<Vec<CleanRange>>()
        })
        .map(|ranges| CleanRange::contained(&ranges[0], &ranges[1]) as usize)
        .sum::<usize>();

    num_of_contained_ranges
}
