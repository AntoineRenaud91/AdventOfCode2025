use std::collections::HashSet;

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
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../data/day04.dat"));
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
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
