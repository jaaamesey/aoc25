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
    let input_str = include_str!("./real_input.txt");
    let mut ranges = Vec::<(u64, u64)>::new();
    for line in input_str.lines() {
        if line.is_empty() {
            break;
        } else {
            let (ls, rs) = line.split_once('-').unwrap();
            let min = ls.parse::<u64>().unwrap();
            let max = rs.parse::<u64>().unwrap();

            // Running the core merging logic this early is not technically necessary, but speeds things up a fair bit.
            // Basically, the first pass eliminates *most* extraneous ranges - it helps to not add those to the array.
            // The rest are fine to skip over as tombstones later.
            if let Some((existing_range_idx, (e_min, e_max))) =
                ranges.iter().enumerate().find(|(_, (e_min, e_max))| {
                    (min >= *e_min && min <= *e_max) || (min >= *e_min && min <= *e_max)
                })
            {
                ranges[existing_range_idx] = (*e_min.min(&min), *e_max.max(&max));
            } else {
                ranges.push((min, max));
            }
        }
    }
    loop {
        let mut num_corrections = 0;
        for idx in 0..ranges.len() {
            let (min, max) = ranges[idx];
            // Slight speedup from skipping tombstones
            if (min, max) == (0, 0) {
                continue;
            }
            if let Some((existing_range_idx, (e_min, e_max))) =
                ranges.iter().enumerate().find(|(e_idx, (e_min, e_max))| {
                    ((min >= *e_min && min <= *e_max) || (min >= *e_min && min <= *e_max))
                        && *e_idx != idx
                })
            {
                ranges[existing_range_idx] = (*e_min.min(&min), *e_max.max(&max));
                // Create tombstone. My input happens to have no ranges with 0 in it, so this is safe.
                ranges[idx] = (0, 0);
                num_corrections += 1;
            }
        }
        if num_corrections == 0 {
            break;
        }
    }
    dbg!(
        ranges
            .iter()
            .map(|(min, max)| if *max == 0 && *min == 0 {
                0
            } else {
                (max - min) + 1
            })
            .sum::<u64>()
    );
}
