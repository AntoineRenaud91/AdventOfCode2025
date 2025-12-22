use std::{collections::HashSet, path::PathBuf, time::Instant};

fn light_pattern_to_u64(s: &str) -> u64 {
    s.chars()
        .enumerate()
        .fold(0, |acc, (i, c)| if c == '#' { acc | (1 << i) } else { acc })
}

#[test]
fn test_light_pattern_to_u64() {
    assert_eq!(light_pattern_to_u64(".##."), 0b110);
    assert_eq!(light_pattern_to_u64("...#."), 0b1000);
    assert_eq!(light_pattern_to_u64(".###.#"), 0b101110);
}

pub fn btn_pattern_to_u64(s: &str) -> u64 {
    s.split(',')
        .map(|num| num.trim().parse::<usize>().unwrap())
        .fold(0u64, |acc, idx| acc | (1 << idx))
}

#[test]
fn test_btn_pattern_to_u64() {
    assert_eq!(btn_pattern_to_u64("3"), 0b1000);
    assert_eq!(btn_pattern_to_u64("1,3"), 0b1010);
    assert_eq!(btn_pattern_to_u64("0,2"), 0b101);
    assert_eq!(btn_pattern_to_u64("4,5"), 0b110000);
}

fn part1(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace().peekable();
            let target = light_pattern_to_u64(
                iter.next()
                    .unwrap()
                    .trim_start_matches('[')
                    .trim_end_matches(']'),
            );
            let btns = {
                let mut b = vec![];
                while iter.peek().is_some() && iter.peek().unwrap().starts_with("(") {
                    b.push(btn_pattern_to_u64(
                        iter.next()
                            .unwrap()
                            .trim_start_matches('(')
                            .trim_end_matches(')'),
                    ));
                }
                b
            };
            let mut current_sets = HashSet::new();
            current_sets.insert(0u64);
            for i in 1..btns.len() {
                current_sets = current_sets
                    .iter()
                    .flat_map(|&lights| btns.iter().map(move |&btn| lights ^ btn))
                    .collect();
                if current_sets.contains(&target) {
                    return i as u64;
                }
            }
            panic!("No solution found for line: {}", line);
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace().skip(1).peekable();
            let mut btns: Vec<Vec<usize>> = vec![];
            while let Some(next_str) = iter.peek()
                && next_str.starts_with("(")
            {
                btns.push(
                    iter.next()
                        .unwrap()
                        .trim_start_matches('(')
                        .trim_end_matches(')')
                        .split(',')
                        .map(|num| num.trim().parse::<usize>().unwrap())
                        .collect(),
                );
            }
            let target: Vec<i64> = iter
                .next()
                .unwrap()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .map(|num| num.trim().parse::<i64>().unwrap())
                .collect();
            let m = target.len();
            let combos: Vec<(Vec<i64>, i64)> = (0..(1 << btns.len()))
                .map(|n| {
                    let mut counter = vec![0i64; m];
                    let mut nb_pressed = 0i64;
                    for (j, btn) in btns.iter().enumerate() {
                        if (n & (1 << j)) != 0 {
                            nb_pressed += 1;
                            for &idx in btn {
                                counter[idx] += 1;
                            }
                        }
                    }
                    (counter, nb_pressed)
                })
                .collect();
            fn solve(counter: &[i64], combos: &[(Vec<i64>, i64)]) -> Option<i64> {
                if counter.iter().all(|&x| x == 0) {
                    return Some(0);
                }
                combos
                    .iter()
                    .filter(|(comb, _)| {
                        comb.iter().zip(counter).all(|(a, b)| a <= b)
                            && comb.iter().zip(counter).all(|(a, b)| a % 2 == b % 2)
                    })
                    .filter_map(|(comb, nb_pressed)| {
                        let next: Vec<i64> = counter
                            .iter()
                            .zip(comb)
                            .map(|(c, cc)| (c - cc) / 2)
                            .collect();
                        solve(&next, combos).map(|rec| 2 * rec + nb_pressed)
                    })
                    .min()
            }
            solve(&target, &combos).unwrap() as u64
        })
        .sum()
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data")
        .join("day10.dat");
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

    const EXAMPLE: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 33);
    }
}
