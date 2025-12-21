use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
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
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data")
        .join("day07.dat");
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
