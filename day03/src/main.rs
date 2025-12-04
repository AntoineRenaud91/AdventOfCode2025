fn find_largest(line: &str, n: usize) -> i64 {
    let mut nums = vec!['0'; n];
    let ll = line.len();
    let mut curr = -1;
    for (k, num) in nums.iter_mut().enumerate() {
        let start = (curr + 1) as usize;
        let stop = ll - n + k;
        for i in start..=stop {
            if line.as_bytes()[i] as char > *num {
                *num = line.as_bytes()[i] as char;
                curr = i as i32;
            }
        }
    }
    String::from_iter(nums).parse::<i64>().unwrap()
}

fn part1(input: &str) -> i64 {
    input.trim().lines().map(|line| find_largest(line, 2)).sum()
}

fn part2(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|line| find_largest(line, 12))
        .sum()
}

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../data/day03.dat"));
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
