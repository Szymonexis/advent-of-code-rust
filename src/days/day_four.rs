pub fn day_four(lines: &Vec<String>) {
    let mut full_hits = 0;
    let mut partial_hits = 0;

    for line in lines {
        let (f_b, f_e, s_b, s_e) = get_sections(line);
        let (f_vec, s_vec) = get_vecs(f_b, f_e, s_b, s_e);

        let (fully, partially) = is_section_contained(&f_vec, &s_vec, &f_b, &s_b, &f_e, &s_e);

        if fully {
            full_hits += 1;
        }

        if partially {
            partial_hits += 1;
        }
    }

    println!("pairs that have a full overlap: {full_hits}");
    println!("pairs that have a partial overlap: {partial_hits}");
}

fn is_section_partially_contained(master: &Vec<bool>) -> bool {
    for item in master {
        if *item {
            return *item;
        }
    }

    return false;
}

fn is_section_fully_contained(master: &Vec<bool>, slave: &Vec<bool>, s: &isize, e: &isize) -> bool {
    for i in (*s as usize)..=(*e as usize) {
        if !master[i] || !slave[i] {
            return false;
        }
    }

    return true;
}

fn is_section_contained(
    vec1: &Vec<bool>,
    vec2: &Vec<bool>,
    s1: &isize,
    s2: &isize,
    e1: &isize,
    e2: &isize,
) -> (bool, bool) {
    if vec1.len() != vec2.len() {
        return (false, false);
    }

    let mut master: Vec<bool> = Vec::new();

    for i in 0..vec1.len() {
        master.push(vec1[i] && vec2[i]);
    }

    return (
        is_section_fully_contained(&master, vec1, s1, e1)
            || is_section_fully_contained(&master, vec2, s2, e2),
        is_section_partially_contained(&master),
    );
}

fn get_vecs(f_b: isize, f_e: isize, s_b: isize, s_e: isize) -> (Vec<bool>, Vec<bool>) {
    let vec_len = (f_e.max(s_e) + 1) as usize;

    let mut f_vec = vec![false; vec_len];
    let mut s_vec = vec![false; vec_len];

    for i in f_b..=f_e {
        f_vec[i as usize] = true;
    }

    for i in s_b..=s_e {
        s_vec[i as usize] = true;
    }

    return (f_vec, s_vec);
}

fn get_sections(line: &String) -> (isize, isize, isize, isize) {
    let div_idx = line.find(',').unwrap();

    let fst_elf_str = line[0..div_idx].to_string();
    let snd_elf_str = line[div_idx + 1..line.len()].to_string();

    let fst_div_idx = fst_elf_str.find('-').unwrap();
    let snd_div_idx = snd_elf_str.find('-').unwrap();

    let fst_elf_begin: isize = fst_elf_str[0..fst_div_idx].to_string().parse().unwrap();
    let fst_elf_end: isize = fst_elf_str[fst_div_idx + 1..fst_elf_str.len()]
        .to_string()
        .parse()
        .unwrap();

    let snd_elf_begin: isize = snd_elf_str[0..snd_div_idx].to_string().parse().unwrap();
    let snd_elf_end: isize = snd_elf_str[snd_div_idx + 1..snd_elf_str.len()]
        .to_string()
        .parse()
        .unwrap();

    return (fst_elf_begin, fst_elf_end, snd_elf_begin, snd_elf_end);
}
