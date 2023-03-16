pub fn day_two(lines: &Vec<String>) {
    let mut counter = 0;

    for line in lines {
        if counter < 10 {
            println!("{line}");
            counter += 1;
        } else {
            break;
        }
    }
}
