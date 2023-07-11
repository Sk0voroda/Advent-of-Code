#[cfg(test)]
mod tests {
    use rust_advert_of_code::solutions_2022::{day01, day02, day03};

    #[test]
    fn day_01_tests() {
        assert_eq!(day01::top_calories(), 70_698);
        assert_eq!(day01::top_three_calories(), 206_643);
        assert_eq!(day01::top_three_calories_itertools(), 206_643);
    }

    #[test]
    fn day_02_tests() {
        assert_eq!(day02::rock_paper_scissors(day02::game_result_p_1), 15_337);
        assert_eq!(day02::rock_paper_scissors(day02::game_result_p_2), 11_696);
    }
    #[test]
    fn day_03_tests() {
        assert_eq!(day03::rucksack_reorganization(), 8_233);
        assert_eq!(day03::rucksack_reorganization_badges(), 2_821);
    }
}
