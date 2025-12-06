pub fn part1() {
    let input_str = include_str!("./real_input.txt");
    let mut input_iter = input_str.lines().rev();
    let column_ops = input_iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<char>>();
    let mut column_tallies = column_ops
        .iter()
        .map(|op| if *op == '+' { 0 } else { 1 })
        .collect::<Vec<u64>>();
    for line in input_iter {
        for (column_idx, cell) in line.split_whitespace().enumerate() {
            let val = cell.parse::<u64>().unwrap();
            if column_ops[column_idx] == '+' {
                column_tallies[column_idx] += val;
            } else {
                column_tallies[column_idx] *= val;
            }
        }
    }
    dbg!(column_tallies.iter().sum::<u64>());
}

pub fn part2() {
    let input_str = include_str!("./test_input.txt");
}
