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
    let mut input_iter = input_str.lines().rev();
    let column_ops = input_iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<char>>();
    let mut columns = column_ops
        .iter()
        .map(|_| Vec::new())
        .collect::<Vec<Vec<Vec<char>>>>();
    for line in input_iter {
        for (column_idx, cell) in line.split_whitespace().enumerate() {
            columns[column_idx].push(cell.chars().collect());
        }
    }
    // PROBLEM: shifting matters. Need to transpose earlier.
    dbg!(
        columns
            .iter_mut()
            .enumerate()
            .map(|(column_idx, column)| {
                let max_digits = column.iter().fold(0, |acc, curr| acc.max(curr.len()));
                let mut tally = if column_ops[column_idx] == '+' { 0 } else { 1 };
                dbg!(column_ops[column_idx]);
                // Pad columns
                for cell in column.iter_mut() {
                    while cell.len() < max_digits {
                        cell.insert(0, '_');
                    }
                }
                dbg!(&column);
                for i in (0..max_digits) {
                    let mut number = 0 as u64;
                    for j in (0..column.len()) {
                        if let Some(digit) = column[j].get(i) {
                            // does this need to be reversed?
                            if let Ok(parsed_digit) = digit.to_string().parse::<u64>() {
                                number = (number * 10) + parsed_digit;
                            }
                        }
                    }
                    if column_ops[column_idx] == '+' {
                        tally += number;
                    } else {
                        tally *= number;
                    }
                    dbg!(number);
                }
                tally
            })
            .sum::<u64>()
    );
}
