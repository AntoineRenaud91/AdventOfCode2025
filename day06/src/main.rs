use std::time::Instant;

use ndarray::Array2;

fn part1(input: &str) -> u64 {
    let mut lines = input.trim().lines().rev();
    let ops: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|c| match c.trim() {
            "+" => 0,
            "*" => 1,
            _ => panic!("Unrecognised {c}"),
        })
        .collect();
    let mut data = ops.clone();
    for line in lines {
        for (i, num) in line
            .split_whitespace()
            .map(|n| n.trim().parse::<u64>().unwrap())
            .enumerate()
        {
            if ops[i] == 0 {
                data[i] += num;
            } else {
                data[i] *= num
            }
        }
    }
    data.into_iter().sum()
}

fn part2(input: &str) -> u64 {
    let n_rows = input.lines().count() - 1;
    let last_line = input.lines().last().unwrap();
    let n_cols = last_line.len();
    let chars_array = Array2::from_shape_vec(
        [n_rows, n_cols],
        input
            .lines()
            .take(n_rows)
            .flat_map(|line| line.chars().rev())
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let mut reversed_input = chars_array
        .t()
        .outer_iter()
        .flat_map(|col| col.into_iter().chain(std::iter::once(&'\n')))
        .collect::<String>();
    reversed_input.pop();
    let ops: Vec<u64> = last_line
        .split_whitespace()
        .map(|c| match c.trim() {
            "+" => 0,
            "*" => 1,
            _ => panic!("Unrecognised {c}"),
        })
        .rev()
        .collect();
    let mut data = ops.clone();
    for (i, nums_str) in reversed_input
        .split(&format!("\n{}\n", " ".repeat(n_rows)))
        .enumerate()
    {
        for num_str in nums_str.lines() {
            let num = num_str.trim().parse::<u64>().unwrap();
            if ops[i] == 0 {
                data[i] += num;
            } else {
                data[i] *= num
            }
        }
    }
    data.into_iter().sum()
}

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../data/day06.dat"));
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

    const EXAMPLE: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 3263827);
    }
}
