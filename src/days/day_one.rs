pub fn day_one(lines: &Vec<String>) {
    let mut elves: Vec<i32> = Vec::new();

    let mut sum = 0;
    for line in lines {
        let line_parsed: i32 = match line.parse::<i32>() {
            Ok(parsed) => parsed,
            Err(_) => -1,
        };

        if line_parsed == -1 {
            elves.push(sum);
            sum = 0;
        } else {
            sum += line_parsed;
        }
    }

    elves.push(sum);

    for elf in &elves {
        println!("{elf}")
    }

    let mut sorted_elves = elves.clone();
    sorted_elves.sort_by(|a, b| b.cmp(a));
    println!("most calories: {:?}", &sorted_elves[0]);
    println!(
        "most calories (top 3 elves): {:?}",
        &sorted_elves[0..3].iter().sum::<i32>()
    );
}
