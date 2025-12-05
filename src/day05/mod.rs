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
}
