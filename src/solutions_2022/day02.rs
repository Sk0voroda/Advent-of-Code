// https://adventofcode.com/2022/day/2

// first player: A - Rock(1 point), B - Paper(2 points), C - Scissors(3 points)
// second player: X - Rock(1 point), Y - Paper(2 points), and Z - Scissors(3 points)
// Rock (A, X) > Scissors(C, Z)
// Scissors(C, Z) > Paper,(B, Y)
// Paper(B, Y) > Rock(A, X)
// lose 0 points, draw 3 points, win 6 points

// todo: refactor
fn game_result(game: &str) -> usize {
    match game {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        _ => panic!("wrong combination"),
    }
}

// part one
pub fn rock_paper_scissors() -> usize {
    let file_data =
        std::fs::read_to_string("inputs/2022/02.txt").expect("puzzle day 2 file is missing");

    let sum = file_data
        .lines()
        .map(|line| game_result(line))
        .sum::<usize>();

    sum
}
