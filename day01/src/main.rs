pub fn part1(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|line| {
            let (lor, num_str) = line.split_at(1);
            let num: i64 = num_str.parse().unwrap();
            match lor {
                "L" => -num,
                "R" => num,
                _ => panic!("Unexpected direction"),
            }
        })
        .fold((50i64, 0u64), |(mut curr, mut count), delta| {
            curr = (curr + delta) % 100;
            if curr == 0 {
                count += 1;
            }
            (curr, count)
        })
        .1
}

pub fn part2(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|line| {
            let (lor, num_str) = line.split_at(1);
            let num: i64 = num_str.parse().unwrap();
            match lor {
                "L" => -num,
                "R" => num,
                _ => panic!("Unexpected direction"),
            }
        })
        .fold((50i64, 0u64), |(mut curr, mut count), delta| {
            curr += delta;
            count += curr.div_euclid(100).unsigned_abs();
            curr = curr.rem_euclid(100);
            (curr, count)
        })
        .1
}

fn main() {
    let input: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../data/day01.dat"));

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 6);
    }
}
