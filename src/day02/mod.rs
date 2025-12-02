pub fn part1() {
    let input_str = include_str!("./real_input.txt");
    let sum = input_str
        .split(",")
        .map(|s| {
            let (start_s, end_s) = s.split_once('-').unwrap();
            let (start, end) = (
                start_s.trim().parse::<i64>().unwrap(),
                end_s.trim().parse::<i64>().unwrap(),
            );
            let mut sum = 0;
            for i in start..=end {
                let s = &i.to_string();
                if s[0..(s.len() / 2)] == s[(s.len() / 2)..s.len()] {
                    sum += i;
                }
            }
            sum
        })
        .sum::<i64>();
    dbg!(sum);
}

pub fn part2() {
    let input_str = include_str!("./real_input.txt");
    let sum = input_str
        .split(",")
        .map(|s| {
            let (start_s, end_s) = s.split_once('-').unwrap();
            let (start, end) = (
                start_s.trim().parse::<i64>().unwrap(),
                end_s.trim().parse::<i64>().unwrap(),
            );
            let mut sum: i64 = 0;
            for i in start..=end {
                let s = &i.to_string();
                let mut pattern = &s[0..0];
                for j in 0..s.len() / 2 {
                    pattern = &s[0..j + 1];
                    let times_repeated = s.len() / (j + 1);
                    // Slight speedup
                    if times_repeated * (j + 1) != s.len() {
                        continue;
                    }
                    let mut repeated_str = String::new();
                    for _ in 0..times_repeated {
                        repeated_str.push_str(&pattern);
                    }
                    if repeated_str == *s {
                        sum += i as i64;
                        break;
                    }
                }
            }
            sum
        })
        .sum::<i64>();
    dbg!(sum);
}
