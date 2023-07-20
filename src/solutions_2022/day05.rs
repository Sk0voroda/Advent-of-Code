// need fn to parse init crate state

#[derive(Debug)]
struct Crate {
    name: char,
    id: usize,
    stack_id: usize,
}

#[derive(Debug)]
struct Stack {
    id: usize,
    crates: Vec<char>,
}

impl Stack {
    fn new(id: usize) -> Self {
        Self {
            id: id,
            crates: Vec::new(),
        }
    }
}

pub fn supply_stacks() -> String {
    let file_data =
        std::fs::read_to_string("inputs/2022/05.txt").expect("puzzle for day 5 file is missing");

    let mut data_iter = file_data.split("\n\n");
    let crate_data = data_iter.next().unwrap();
    let moves_data = data_iter.next().unwrap();

    println!("{}", crate_data);

    // TODO: move to separate parse func
    let num_of_stacks: usize = crate_data
        .lines()
        .rev()
        .take(1)
        .collect::<String>()
        .replace(' ', "")
        .len();

    let mut stacks = Vec::<Stack>::with_capacity(num_of_stacks);
    for i in 0..num_of_stacks {
        stacks.push(Stack::new(i));
    }

    // removing last indexing line before crate parsing
    let crate_vec = crate_data.lines().collect::<Vec<&str>>();
    let crate_vec = crate_vec[..crate_vec.len() - 1].to_vec();
    // filtering redundant chars from lines
    let crate_vec = crate_vec
        .iter()
        .map(|line| line.replace("[", "").replace("]", "").replace("  ", " "))
        .collect::<Vec<_>>();

    crate_vec.iter().for_each(|line| {
        line.chars().step_by(2).enumerate().for_each(|(i, ch)| {
            if ch.is_ascii_alphabetic() {
                stacks[i].crates.push(ch);
            }
        });
    });

    moves_data.lines().for_each(|line| {
        let mut moves = line
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|n| n.parse::<usize>().unwrap());

        let nums = moves.next().unwrap_or(0);
        let from = moves.next().unwrap_or(0) - 1;
        let to = moves.next().unwrap_or(0) - 1;

        // stacks[from].crates.drain(range);
        for _ in 0..nums {
            let tmp = stacks[from].crates.remove(0);
            stacks[to].crates.insert(0, tmp);

            println!("from {:?}", stacks[from]);
            println!("to {:?}", stacks[to]);
        }
    });

    let result = stacks
        .iter()
        .map(|stack| stack.crates[0])
        .collect::<String>();

    println!("{}", result);
    result
}
