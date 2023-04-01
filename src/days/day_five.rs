use regex::Regex;

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl Move {
    pub fn new(amount: usize, from: usize, to: usize) -> Move {
        Move { amount, from, to }
    }
}

pub fn day_five(lines: &Vec<String>) {
    let mut moves: Vec<Move> = vec![];
    let mut crates_one: Vec<Vec<char>> = initial_crates();
    let mut crates_two: Vec<Vec<char>> = initial_crates();

    for line in lines {
        moves.push(get_move(line));
    }

    for mv in moves {
        crates_one = move_one_at_a_time(&mv, crates_one);
        crates_two = move_multiple_at_a_time(&mv, crates_two)
    }

    let mut output_one = String::new();
    for i in 0..crates_one.len() {
        let top_crate = crates_one[i].pop().unwrap();
        output_one.push(top_crate);
    }

    let mut output_two = String::new();
    for i in 0..crates_two.len() {
        let top_crate = crates_two[i].pop().unwrap();
        output_two.push(top_crate);
    }

    println!("top crates while moving one at a time are: {}", output_one);
    println!(
        "top crates while moving multiple at a time are: {}",
        output_two
    );
}

fn move_multiple_at_a_time(mv: &Move, mut crates: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let from_index = mv.from - 1;
    let to_index = mv.to - 1;
    let amount = mv.amount;

    let start_index = crates[from_index].len() - amount;
    let end_index = crates[from_index].len();

    let mut crates_to_move: Vec<char> = Vec::new();
    for cr in &crates[from_index][start_index..end_index] {
        crates_to_move.push((*cr).clone());
    }

    for _ in 0..amount {
        crates[from_index].pop();
    }

    for cr in crates_to_move {
        crates[to_index].push(cr);
    }

    return crates;
}

fn move_one_at_a_time(mv: &Move, mut crates: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let from_index = mv.from - 1;
    let to_index = mv.to - 1;
    let amount = mv.amount;

    for _ in 0..amount {
        let crate_to_move = crates[from_index].pop().unwrap();
        crates[to_index].push(crate_to_move);
    }

    return crates;
}

fn get_move(line: &String) -> Move {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let caps = re.captures(line).unwrap();

    let amount: usize = caps[1].parse().unwrap();
    let from: usize = caps[2].parse().unwrap();
    let to: usize = caps[3].parse().unwrap();

    return Move::new(amount, from, to);
}

fn initial_crates() -> Vec<Vec<char>> {
    // [S]                 [T] [Q]
    // [L]             [B] [M] [P]     [T]
    // [F]     [S]     [Z] [N] [S]     [R]
    // [Z] [R] [N]     [R] [D] [F]     [V]
    // [D] [Z] [H] [J] [W] [G] [W]     [G]
    // [B] [M] [C] [F] [H] [Z] [N] [R] [L]
    // [R] [B] [L] [C] [G] [J] [L] [Z] [C]
    // [H] [T] [Z] [S] [P] [V] [G] [M] [M]
    // 1   2   3   4   5   6   7   8   9

    let crates: Vec<Vec<char>> = vec![
        vec!['H', 'R', 'B', 'D', 'Z', 'F', 'L', 'S'],
        vec!['T', 'B', 'M', 'Z', 'R'],
        vec!['Z', 'L', 'C', 'H', 'N', 'S'],
        vec!['S', 'C', 'F', 'J'],
        vec!['P', 'G', 'H', 'W', 'R', 'Z', 'B'],
        vec!['V', 'J', 'Z', 'G', 'D', 'N', 'M', 'T'],
        vec!['G', 'L', 'N', 'W', 'F', 'S', 'P', 'Q'],
        vec!['M', 'Z', 'R'],
        vec!['M', 'C', 'L', 'G', 'V', 'R', 'T'],
    ];

    return crates;
}
