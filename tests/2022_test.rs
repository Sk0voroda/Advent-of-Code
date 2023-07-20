#[cfg(test)]
mod tests {
    use rust_advert_of_code::solutions_2022::{day01, day02, day03, day04, day05};

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
    #[test]
    fn day_04_tests() {
        // ClearRange contains tests
        let base = day04::clean_range::CleanRange::new(1, 10);
        let full = day04::clean_range::CleanRange::new(5, 8);
        let partial_1 = day04::clean_range::CleanRange::new(0, 4);
        let partial_2 = day04::clean_range::CleanRange::new(7, 12);
        let not_contained_1 = day04::clean_range::CleanRange::new(12, 15);

        assert_eq!(base.contains(&full), true);
        assert_eq!(base.contains(&partial_1), false);
        assert_eq!(base.contains(&partial_2), false);
        assert_eq!(base.contains(&not_contained_1), false);
        assert_eq!(day04::camp_cleanup(), 503);
        assert_eq!(day04::camp_cleanup_overlaps(), 827);
    }
    #[test]
    fn day_05_tests() {
        assert_eq!(day05::supply_stacks(), "ZWHVFWQWW");
        // assert_eq!(day05::rucksack_reorganization_badges(), 2_821);
    }
}
