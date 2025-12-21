use std::{collections::HashSet, path::PathBuf, time::Instant};

const NEIGHTBORD: [(i64, i64); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

fn parse(input: &str) -> HashSet<(i64, i64)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.trim().chars().enumerate().filter_map(move |(j, c)| {
                if c == '@' {
                    Some((i as i64, j as i64))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn part1(input: &str) -> u64 {
    let pos = parse(input);
    pos.iter()
        .filter(|&&(i, j)| {
            NEIGHTBORD
                .into_iter()
                .filter(|(di, dj)| pos.contains(&(i + di, j + dj)))
                .count()
                < 4
        })
        .count() as u64
}

fn part2(input: &str) -> u64 {
    let mut grid = parse(input);
    let mut count = 0;
    loop {
        let to_remove = grid
            .iter()
            .filter(|&&(i, j)| {
                NEIGHTBORD
                    .into_iter()
                    .filter(|(di, dj)| grid.contains(&(i + di, j + dj)))
                    .count()
                    < 4
            })
            .cloned()
            .collect::<Vec<_>>();
        if to_remove.is_empty() {
            return count as u64;
        }
        count += to_remove.len();
        for pos in to_remove {
            grid.remove(&pos);
        }
    }
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data")
        .join("day04.dat");
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

    const EXAMPLE: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 43);
    }
}
