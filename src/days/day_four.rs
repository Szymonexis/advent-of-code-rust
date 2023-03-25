pub fn day_four(lines: &Vec<String>) {
    let mut hits = 0;

    for line in lines {
        let (f_b, f_e, s_b, s_e) = get_sections(line);
        let (f_vec, s_vec) = get_vecs(f_b, f_e, s_b, s_e);

        if is_section_contained(&f_vec, &s_vec) {
            hits += 1;
        }
    }

    println!("pairs that have a full overlap: {hits}");
}

fn is_section_contained(vec1: &Vec<bool>, vec2: &Vec<bool>) -> bool {
    for i in 0..vec1.len() {
        if vec1[i] {
            let slice1 = &vec1[i..];
            if slice1.len() > vec2.len() {
                break;
            }

            let slice2 = &vec2[i..(i + slice1.len())];
            if slice1 == slice2 {
                return true;
            }
        }
    }
    return false;
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
