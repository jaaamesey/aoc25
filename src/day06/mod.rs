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
    let input_str = include_str!("./real_input.txt");

    let mut input_iter = input_str.lines().rev();
    let ops = input_iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<char>>();

    let empty_vec = Vec::with_capacity(0);

    let old_grid: Vec<Vec<char>> = input_iter.map(|line| line.chars().collect()).collect();
    let max_old_line_length = old_grid.iter().fold(0, |acc, line| line.len().max(acc));

    let transposed_grid = (0..max_old_line_length).map(|x| {
        (0..old_grid.len())
            .map(|y| *old_grid.get(y).unwrap_or(&empty_vec).get(x).unwrap_or(&'_'))
            .rev()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    });

    let mut index_in_ops = 0;
    let mut outer_tally = 0;
    let mut current_tally = if ops[index_in_ops] == '+' { 0 } else { 1 };
    for line in transposed_grid {
        if line.is_empty() {
            index_in_ops += 1;
            outer_tally += current_tally;
            current_tally = if ops[index_in_ops] == '+' { 0 } else { 1 };
            continue;
        }
        let num = line.parse::<u64>().unwrap();
        if ops[index_in_ops] == '+' {
            current_tally += num;
        } else {
            current_tally *= num;
        }
    }
    outer_tally += current_tally;

    dbg!(&outer_tally);
}
