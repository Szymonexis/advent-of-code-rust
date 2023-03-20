struct FirstStratHelp {
    pub elf_moves: Vec<char>,
    pub my_moves: Vec<char>,
    pub outcomes: Vec<Vec<usize>>,
}

impl FirstStratHelp {
    pub fn new() -> FirstStratHelp {
        FirstStratHelp {
            elf_moves: vec!['A', 'B', 'C'],
            my_moves: vec!['X', 'Y', 'Z'],
            outcomes: vec![vec![3, 0, 6], vec![6, 3, 0], vec![0, 6, 3]],
        }
    }
}

struct SecondStratHelp {
    pub elf_moves: Vec<char>,
    pub outcomes: Vec<char>,
    pub my_moves: Vec<Vec<usize>>,
}

impl SecondStratHelp {
    pub fn new() -> SecondStratHelp {
        SecondStratHelp {
            elf_moves: vec!['A', 'B', 'C'],
            outcomes: vec!['X', 'Y', 'Z'],
            my_moves: vec![vec![2, 0, 1], vec![0, 1, 2], vec![1, 2, 0]],
        }
    }
}

pub fn day_two(lines: &Vec<String>) {
    let first_strat_help = FirstStratHelp::new();
    let second_strat_help = SecondStratHelp::new();

    let mut first_strat_total = 0;
    let mut second_strat_total = 0;

    for line in lines {
        let (my_first_score, first_outcome) = first_strat_calcs(line, &first_strat_help);
        let (my_second_score, second_outcome) = second_strat_calcs(line, &second_strat_help);

        first_strat_total += my_first_score + first_outcome;
        second_strat_total += my_second_score + second_outcome;
    }

    println!("my total score (first strategy): {}", first_strat_total);
    println!("my total score (second strategy): {}", second_strat_total);
}

fn second_strat_calcs(line: &String, help: &SecondStratHelp) -> (usize, usize) {
    let elf_choice = line.chars().nth(0).unwrap();
    let outcome_char = line.chars().nth(2).unwrap();

    let elf_index = get_index(&help.elf_moves, &elf_choice);
    let outcome_index = get_index(&help.outcomes, &outcome_char);

    let my_choice = help.my_moves[outcome_index][elf_index];

    let outcome = outcome_index * 3;
    let my_score = my_choice + 1;

    return (my_score, outcome);
}

fn first_strat_calcs(line: &String, help: &FirstStratHelp) -> (usize, usize) {
    let elf_choice = line.chars().nth(0).unwrap();
    let my_choice = line.chars().nth(2).unwrap();

    let elf_index = get_index(&help.elf_moves, &elf_choice);
    let my_index = get_index(&help.my_moves, &my_choice);

    let outcome = help.outcomes[my_index][elf_index];
    let my_score = my_index + 1;

    return (my_score, outcome);
}

fn get_index(c_vec: &Vec<char>, c: &char) -> usize {
    return c_vec.iter().position(|&r| r == *c).unwrap();
}
