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

fn compare(first_range: &str, second_range: &str) {
    todo!("implement")
}

pub fn camp_cleanup() -> usize {
    let file_data =
        std::fs::read_to_string("inputs/2022/04.txt").expect("puzzle for day 4 file is missing");

    let ranges_vec = file_data
        .lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    println!("{:?}", ranges_vec);
    // let fully_containes = ranges_vec.iter().map(|ranges| {});
    0
}
