pub fn day_six(lines: &Vec<String>) {
    let line = &lines[0];

    let marker_length = 4;
    let message_length = 14;

    let marker_index = find_sequence(line, marker_length);
    let message_index = find_sequence(line, message_length);

    println!("marker index: {}", marker_index + (marker_length - 1));
    println!("message index: {}", message_index + (message_length - 1));
}

fn find_sequence(s: &String, length: usize) -> usize {
    let mut string_chars: Vec<char> = Vec::new();

    for c in s.chars() {
        string_chars.push(c);
    }

    let mut sequence = &string_chars[0..length];
    for i in length..string_chars.len() {
        if check_if_sequence_unique(sequence.to_vec()) {
            return i;
        }

        sequence = &string_chars[i..length + i]
    }

    return 0;
}

fn check_if_sequence_unique(sequence: Vec<char>) -> bool {
    let mut unique = true;

    for (i, c) in sequence.iter().enumerate() {
        for (j, k) in sequence.iter().enumerate() {
            if j != i {
                unique = unique && (c != k);
            }
        }
    }

    return unique;
}
