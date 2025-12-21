use std::{path::PathBuf, time::Instant};

pub fn part1(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .map(|data| {
            let (start_str, end_str) = data.split_once('-').unwrap();
            let start: u64 = start_str.parse().unwrap();
            let end: u64 = end_str.parse().unwrap();
            (start..=end)
                .filter(|&num| {
                    let num_str = num.to_string();
                    let len = num_str.len();
                    if !len.is_multiple_of(2) {
                        return false;
                    }
                    let mid = len / 2;
                    let first_half = &num_str[..mid];
                    let second_half = &num_str[mid..];
                    first_half == second_half && !first_half.starts_with('0')
                })
                .sum::<u64>()
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .map(|data| {
            let (start_str, end_str) = data.split_once('-').unwrap();
            let start: u64 = start_str.parse().unwrap();
            let end: u64 = end_str.parse().unwrap();
            (start..=end)
                .filter(|&num| {
                    let num_str = num.to_string();
                    let len = num_str.len();
                    for pattern_len in 1..=len / 2 {
                        if len % pattern_len == 0 {
                            let pattern = &num_str[..pattern_len];
                            if pattern.starts_with('0') {
                                continue;
                            }
                            let mut is_repeated = true;
                            for i in (pattern_len..len).step_by(pattern_len) {
                                if &num_str[i..i + pattern_len] != pattern {
                                    is_repeated = false;
                                    break;
                                }
                            }

                            if is_repeated {
                                return true;
                            }
                        }
                    }
                    false
                })
                .sum::<u64>()
        })
        .sum()
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data")
        .join("day02.dat");
    let input = std::fs::read_to_string(path).unwrap();
    let t = Instant::now();
    println!("Part 1: {}", part1(&input));
    println!("Part 1 took: {:?}", t.elapsed());
    let t = Instant::now();
    println!("Part 2: {}", part2(&input));
    println!("Part 2 took: {:?}", t.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4174379265);
    }
}
