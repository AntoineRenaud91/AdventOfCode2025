use std::{path::PathBuf, time::Instant};

fn get_red_tile_pos(input: &str) -> impl Iterator<Item = [i64; 2]> + '_ {
    input.trim().lines().map(|line| {
        let (i_str, j_str) = line.split_once(',').unwrap();
        let i: i64 = i_str.trim().parse().unwrap();
        let j: i64 = j_str.trim().parse().unwrap();
        [i, j]
    })
}

fn part1(input: &str) -> u64 {
    let pos = get_red_tile_pos(input).collect::<Vec<_>>();
    pos.iter()
        .enumerate()
        .flat_map(|(i, p1)| {
            pos[i + 1..].iter().map(move |p2| {
                (
                    p1,
                    p2,
                    ((p2[0] - p1[0]).unsigned_abs() + 1) * ((p2[1] - p1[1]).unsigned_abs() + 1),
                )
            })
        })
        .max_by_key(|&(_, _, area)| area)
        .inspect(|(p1, p2, _)| {
            println!("max between {:?} and {:?}", p1, p2);
        })
        .unwrap()
        .2
}

#[cfg(test)]
fn printgrid(pos: &[[i64; 2]], p1: &[i64; 2], p2: &[i64; 2]) {
    let i_max = pos.iter().map(|p| p[0]).max().unwrap();
    let j_max = pos.iter().map(|p| p[1]).max().unwrap();
    let mut grid = vec![vec!['.'; (j_max + 2) as usize]; (i_max + 2) as usize];
    for pslice in pos
        .windows(2)
        .chain([pos[pos.len() - 1], pos[0]].windows(2))
    {
        let p1 = pslice[0];
        let p2 = pslice[1];
        grid[p1[0] as usize][p1[1] as usize] = '█';
        if p1[0] == p2[0] {
            for j in p1[1].min(p2[1]) + 1..=p1[1].max(p2[1]) {
                grid[p1[0] as usize][j as usize] = '█';
            }
        } else {
            for i in p1[0].min(p2[0]) + 1..=p1[0].max(p2[0]) {
                grid[i as usize][p1[1] as usize] = '█';
            }
        }
    }
    grid[p1[0] as usize][p1[1] as usize] = 'X';
    grid[p2[0] as usize][p2[1] as usize] = 'X';
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn part2(input: &str) -> u64 {
    let pos = get_red_tile_pos(input).collect::<Vec<_>>();
    let mut j_lines = vec![];
    let mut i_lines = vec![];
    for pos_slice in pos
        .windows(2)
        .chain([pos[pos.len() - 1], pos[0]].windows(2))
    {
        let p1 = pos_slice[0];
        let p2 = pos_slice[1];
        if p1[0] == p2[0] {
            if p1[1] > p2[1] {
                i_lines.push([p2, p1]);
            } else {
                i_lines.push([p1, p2]);
            }
        } else if p1[0] > p2[0] {
            j_lines.push([p2, p1]);
        } else {
            j_lines.push([p1, p2]);
        }
    }
    j_lines.sort_by_key(|line| line[0][0]);
    i_lines.sort_by_key(|line| line[0][1]);
    pos.iter()
        .enumerate()
        .flat_map(|(i, p1)| pos[i + 1..].iter().map(move |p2| [p1, p2]))
        .filter_map(|[p1, p2]| {
            let (i_min, i_max) = if p1[0] < p2[0] {
                (p1[0], p2[0])
            } else {
                (p2[0], p1[0])
            };
            let (j_min, j_max) = if p1[1] < p2[1] {
                (p1[1], p2[1])
            } else {
                (p2[1], p1[1])
            };
            let rect_i_range_excl = i_min + 1..=i_max - 1;
            let rect_i_range_incl = i_min..=i_max;
            let rect_j_range_excl = j_min + 1..=j_max - 1;
            let rect_j_range_incl = j_min..=j_max;
            // check if rect edges intersect any poly lines
            if j_lines.iter().any(|j_line| {
                let line_i_range = j_line[0][0] + 1..j_line[1][0];
                rect_j_range_excl.contains(&j_line[0][1]) && {
                    line_i_range.contains(&i_min)
                        || line_i_range.contains(&i_max)
                        || (j_line[0][0] == i_min && rect_i_range_incl.contains(&j_line[1][0]))
                        || (j_line[1][0] == i_max && rect_i_range_incl.contains(&j_line[0][0]))
                }
            }) || i_lines.iter().any(|i_line| {
                let line_j_range = i_line[0][1] + 1..i_line[1][1];
                rect_i_range_excl.contains(&i_line[0][0]) && {
                    line_j_range.contains(&j_min)
                        || line_j_range.contains(&j_max)
                        || (i_line[0][1] == j_min && rect_j_range_incl.contains(&i_line[1][1]))
                        || (i_line[1][1] == j_max && rect_j_range_incl.contains(&i_line[0][1]))
                }
            }) {
                None?
            }
            // check if mid point is inside
            let mid_point = [(i_min + i_max) / 2, (j_min + j_max) / 2];
            if j_lines
                .iter()
                .filter(|line| line[0][1] > mid_point[1])
                .filter(|line| (line[0][0]..line[1][0]).contains(&mid_point[0]))
                .count()
                % 2
                == 0
            {
                None?
            }
            Some((
                p1,
                p2,
                ((p2[0] - p1[0]).unsigned_abs() + 1) * ((p2[1] - p1[1]).unsigned_abs() + 1),
            ))
        })
        .max_by_key(|&(_, _, area)| area)
        .inspect(|(p1, p2, _)| {
            println!("max between {:?} and {:?}", p1, p2);
            #[cfg(test)]
            printgrid(&pos, p1, p2)
        })
        .unwrap()
        .2
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data")
        .join("day09.dat");
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
    fn flipv(input: &str) -> String {
        let mut out = String::with_capacity(input.len());
        for line in input.trim().lines().rev() {
            out.push_str(line);
            out.push('\n');
        }
        out
    }
    fn fliph(input: &str) -> String {
        let mut out = String::with_capacity(input.len());
        for line in input.trim().lines() {
            let rev_line: String = line.chars().rev().collect();
            out.push_str(&rev_line);
            out.push('\n');
        }
        out
    }
    fn flip_ij(input: &str) -> String {
        let mut out = String::with_capacity(input.len());
        for line in input.trim().lines() {
            let (i_str, j_str) = line.split_once(',').unwrap();
            out.push_str(j_str);
            out.push(',');
            out.push_str(i_str);
            out.push('\n');
        }
        out
    }

    const EXAMPLE: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 50);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 24);
        assert_eq!(part2(&flipv(EXAMPLE)), 24);
        assert_eq!(part2(&fliph(EXAMPLE)), 24);
        assert_eq!(part2(&flip_ij(EXAMPLE)), 24);
        assert_eq!(part2(&flip_ij(&flipv(EXAMPLE))), 24);
        assert_eq!(part2(&flip_ij(&fliph(EXAMPLE))), 24);
    }
}
