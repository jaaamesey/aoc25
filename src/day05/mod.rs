pub fn part1() {
    let input_str = include_str!("./real_input.txt");
    let mut now_parsing_available = false;
    let mut ranges = Vec::<(u64, u64)>::new();
    let mut fresh_count = 0;
    for line in input_str.lines() {
        if line.is_empty() {
            now_parsing_available = true;
        } else if now_parsing_available {
            let id = line.parse::<u64>().unwrap();
            if ranges.iter().any(|&(min, max)| id >= min && id <= max) {
                fresh_count += 1;
            }
        } else {
            let (ls, rs) = line.split_once('-').unwrap();
            ranges.push((ls.parse().unwrap(), rs.parse().unwrap()));
        }
    }
    dbg!(fresh_count);
}

pub fn part2() {
    let input_str = include_str!("./test_input.txt");
    let mut ranges = Vec::<(u64, u64)>::new();
    for line in input_str.lines() {
        if line.is_empty() {
            break;
        } else {
            let (ls, rs) = line.split_once('-').unwrap();
            let min = ls.parse::<u64>().unwrap();
            let max = rs.parse::<u64>().unwrap();

            if let Some((existing_range_idx, (e_min, e_max))) =
                ranges.iter().enumerate().find(|(_, (e_min, e_max))| {
                    (min >= *e_min && min <= *e_max) || (min >= *e_min && min <= *e_max)
                })
            {
                dbg!(
                    "Merging: ",
                    (e_min, e_max),
                    (min, max),
                    "to",
                    (*e_min.min(&min), *e_max.max(&max))
                );
                ranges[existing_range_idx] = (*e_min.min(&min), *e_max.max(&max));
            } else {
                ranges.push((min, max));
            }
        }
    }
    //dbg!(&ranges);

    dbg!(ranges.iter().map(|(min, max)| max - min).sum::<u64>());
}
