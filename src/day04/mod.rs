pub fn part1() {
    let input_str = include_str!("./real_input.txt");
    let grid = input_str
        .lines()
        // TODO: don't use chars
        .map(|l| l.chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let dirs: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut outer_count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &is_paper) in row.iter().enumerate() {
            if !is_paper {
                continue;
            }
            let mut count = 0;
            for (dy, dx) in dirs {
                if (y < 1 && dy < 0)
                    || (x < 1 && dx < 0)
                    || (y > grid.len() - 2 && dy > 0)
                    || (x > row.len() - 2 && dx > 0)
                {
                    continue;
                }
                if grid[y.checked_add_signed(dy).unwrap()][x.checked_add_signed(dx).unwrap()] {
                    count += 1;
                }
            }
            if count > 3 {
                continue;
            }
            outer_count += 1;
        }
    }

    dbg!(outer_count);
}

pub fn part2() {
    let input_str = include_str!("./real_input.txt");
    let mut grid = input_str
        .lines()
        // TODO: don't use chars
        .map(|l| l.chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let dirs: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut outer_count = 0;
    let mut positions_to_clear: Vec<(usize, usize)> = Vec::new();

    loop {
        let prev_outer_count = outer_count;
        for (y, row) in grid.iter().enumerate() {
            for (x, &is_paper) in row.iter().enumerate() {
                if !is_paper {
                    continue;
                }
                let mut count = 0;
                for (dy, dx) in dirs {
                    if (y < 1 && dy < 0)
                        || (x < 1 && dx < 0)
                        || (y > grid.len() - 2 && dy > 0)
                        || (x > row.len() - 2 && dx > 0)
                    {
                        continue;
                    }
                    if grid[y.checked_add_signed(dy).unwrap()][x.checked_add_signed(dx).unwrap()] {
                        count += 1;
                    }
                }
                if count > 3 {
                    continue;
                }
                outer_count += 1;
                positions_to_clear.push((y, x));
            }
        }
        for &(y, x) in positions_to_clear.iter() {
            grid[y][x] = false;
        }
        positions_to_clear.clear();
        if prev_outer_count == outer_count {
            break;
        }
    }

    dbg!(outer_count);
}
