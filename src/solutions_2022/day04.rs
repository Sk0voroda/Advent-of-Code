pub mod clean_range {
    use std::fmt::Display;

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
        pub fn new_str(start: &str, end: &str) -> Self {
            let start = start.parse::<usize>().unwrap();
            let end = end.parse::<usize>().unwrap();

            CleanRange::new(start, end)
        }
        pub fn contains(&self, other: &Self) -> bool {
            self.start <= other.start && other.end <= self.end
        }
        pub fn contained(first: &CleanRange, second: &CleanRange) -> bool {
            first.contains(&second) || second.contains(&first)
        }
    }

    impl Display for CleanRange {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CleanRange: {}...{}", self.start, self.end)
        }
    }
}

use clean_range::*;

pub fn camp_cleanup() -> usize {
    let file_data =
        std::fs::read_to_string("inputs/2022/04.txt").expect("puzzle for day 4 file is missing");

    let num_of_contained_ranges = file_data
        .lines()
        // cast line 2-5,4-5 to vec<CleanRange>
        .map(|line| {
            line.split(',')
                // map range into CleanRange object
                .map(|ranges| {
                    let mut range_values = ranges.split('-');
                    let start = range_values.next().unwrap();
                    let end = range_values.next().unwrap();

                    CleanRange::new_str(start, end)
                })
                .collect::<Vec<CleanRange>>()
        })
        .map(|ranges| CleanRange::contained(&ranges[0], &ranges[1]) as usize)
        .sum::<usize>();

    num_of_contained_ranges
}
