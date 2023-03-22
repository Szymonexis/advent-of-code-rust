struct Helper {
    pub ascii_lower_upper: [char; 52],
}

impl Helper {
    pub fn new() -> Helper {
        Helper {
            ascii_lower_upper: [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F',
                'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
                'W', 'X', 'Y', 'Z',
            ],
        }
    }
}

pub fn day_three(lines: &Vec<String>) {
    let helper = Helper::new();

    let mut first_part_score: usize = 0;
    let mut second_part_score: usize = 0;

    for line in lines {
        first_part_score = calc_first_part(line, first_part_score, &helper)
    }

    for index in (0..lines.len()).step_by(3) {
        second_part_score = calc_second_part(lines, &index, second_part_score, &helper);
    }

    println!("first_part_score: {first_part_score}");
    println!("second_part_score: {second_part_score}");
}

fn calc_second_part(
    lines: &Vec<String>,
    index: &usize,
    mut second_part_score: usize,
    helper: &Helper,
) -> usize {
    let end = (*index + 3).min(lines.len());
    let slice = &lines[*index..end];

    let common_char = match second_part_identify_common_sign(&slice[0], &slice[1], &slice[2]) {
        Some(some_common_char) => some_common_char,
        None => ' ',
    };

    second_part_score += get_char_score(&common_char, helper);
    return second_part_score;
}

fn second_part_identify_common_sign(s1: &String, s2: &String, s3: &String) -> Option<char> {
    for c in s1.chars() {
        if s2.contains(c) && s3.contains(c) {
            return Some(c);
        }
    }
    None
}

fn calc_first_part(line: &String, mut first_part_score: usize, helper: &Helper) -> usize {
    let line_length = line.len();

    if line_length % 2 == 0 {
        let mid_index = line_length / 2;
        let first_part = &line[0..mid_index];
        let second_part = &line[mid_index..line_length];

        let first_chars: Vec<char> = first_part.chars().collect();
        let second_chars: Vec<char> = second_part.chars().collect();

        let common_char = match first_part_identify_common_sign(&first_chars, &second_chars) {
            Some(some_common_char) => some_common_char,
            None => ' ',
        };

        first_part_score += get_char_score(&common_char, &helper);
    }

    return first_part_score;
}

fn first_part_identify_common_sign(
    first_part: &Vec<char>,
    second_part: &Vec<char>,
) -> Option<char> {
    for f_c in first_part {
        for s_c in second_part {
            if *f_c == *s_c {
                return Some(*f_c);
            }
        }
    }

    return None;
}

fn get_char_score(char: &char, helper: &Helper) -> usize {
    return helper
        .ascii_lower_upper
        .iter()
        .position(|&r| r == *char)
        .unwrap()
        + 1;
}
