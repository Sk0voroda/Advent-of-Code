mod solutions_2022;
use solutions_2022::{day01, day02};

fn main() {
    println!("Day 1 - {} result", day01::top_three_calories_itertools());

    println!(
        "Day 2 - {} result",
        day02::rock_paper_scissors(day02::game_result_p_2)
    );
}
