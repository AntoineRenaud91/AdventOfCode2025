use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn part1(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let mut beams = HashSet::new();
    let mut split = 0;
    beams.insert(
        lines
            .next()
            .unwrap()
            .chars()
            .enumerate()
            .find(|(_, c)| c == &'S')
            .unwrap()
            .0,
    );
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '^' && beams.remove(&i) {
                beams.insert(i - 1);
                beams.insert(i + 1);
                split += 1;
            }
        }
    }
    split
}

fn part2(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let mut beams = HashMap::new();
    beams.insert(
        lines
            .next()
            .unwrap()
            .chars()
            .enumerate()
            .find(|(_, c)| c == &'S')
            .unwrap()
            .0,
        1,
    );
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '^'
                && let Some(count) = beams.remove(&i)
            {
                *beams.entry(i - 1).or_default() += count;
                *beams.entry(i + 1).or_default() += count;
            }
        }
    }
    beams.values().sum()
}

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../data/day07.dat"));
    let t = Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Part 1 took: {:?}", t.elapsed());
    let t = Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Part 2 took: {:?}", t.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 40);
    }
}
