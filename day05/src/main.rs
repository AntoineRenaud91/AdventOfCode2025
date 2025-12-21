use std::{ops::RangeInclusive, path::PathBuf, time::Instant};

enum QueryResult {
    In(usize),
    Out(usize),
}

#[derive(Default)]
struct NonOverlappingRanges {
    ranges: Vec<RangeInclusive<u64>>,
}

impl NonOverlappingRanges {
    fn query(&self, value: &u64) -> QueryResult {
        match self.ranges.binary_search_by(|r| r.start().cmp(value)) {
            Ok(i) => QueryResult::In(i),
            Err(i) => {
                if i == 0 {
                    QueryResult::Out(0)
                } else if self.ranges[i - 1].contains(value) {
                    QueryResult::In(i - 1)
                } else {
                    QueryResult::Out(i)
                }
            }
        }
    }

    fn count(&self) -> u64 {
        self.ranges.iter().map(|r| r.end() - r.start() + 1).sum()
    }

    fn contains(&self, value: &u64) -> bool {
        match self.query(value) {
            QueryResult::In(_) => true,
            QueryResult::Out(_) => false,
        }
    }

    fn insert(&mut self, range: RangeInclusive<u64>) {
        match (self.query(range.start()), self.query(range.end())) {
            (QueryResult::In(i_start), QueryResult::In(i_end)) => {
                if i_start < i_end {
                    self.ranges[i_start] =
                        *self.ranges[i_start].start()..=*self.ranges[i_end].end();
                    self.ranges.drain(i_start + 1..=i_end);
                }
            }
            (QueryResult::In(i_start), QueryResult::Out(i_end)) => {
                self.ranges[i_start] = *self.ranges[i_start].start()..=*range.end();
                if i_start < i_end {
                    self.ranges.drain(i_start + 1..i_end);
                }
            }
            (QueryResult::Out(i_start), QueryResult::In(i_end)) => {
                self.ranges[i_end] = *range.start()..=*self.ranges[i_end].end();
                if i_start + 1 < i_end {
                    self.ranges.drain(i_start + 1..=i_end - 1);
                }
            }
            (QueryResult::Out(i_start), QueryResult::Out(i_end)) => {
                if i_start == i_end {
                    self.ranges.insert(i_start, range);
                } else {
                    self.ranges.drain(i_start..i_end);
                    self.ranges.insert(i_start, range);
                }
            }
        }
    }
}

#[test]
fn test_no_range() {
    let mut no_ranges = NonOverlappingRanges::default();
    no_ranges.insert(1..=2);
    assert_eq!(no_ranges.ranges, [1..=2]);
    no_ranges.insert(7..=8);
    assert_eq!(no_ranges.ranges, [1..=2, 7..=8]);
    no_ranges.insert(4..=5);
    assert_eq!(no_ranges.ranges, [1..=2, 4..=5, 7..=8]);
    no_ranges.insert(2..=3);
    assert_eq!(no_ranges.ranges, [1..=3, 4..=5, 7..=8]);
    no_ranges.insert(0..=1);
    assert_eq!(no_ranges.ranges, [0..=3, 4..=5, 7..=8]);
    no_ranges.insert(6..=9);
    assert_eq!(no_ranges.ranges, [0..=3, 4..=5, 6..=9]);
    no_ranges.insert(1..=4);
    assert_eq!(no_ranges.ranges, [0..=5, 6..=9]);
    no_ranges = NonOverlappingRanges::default();
    no_ranges.insert(3..=4);
    no_ranges.insert(8..=9);
    no_ranges.insert(7..=8);
    assert_eq!(no_ranges.ranges, [3..=4, 7..=9]);
    no_ranges.insert(1..=10);
    assert_eq!(no_ranges.ranges, [1..=10]);
}

fn part1(input: &str) -> u64 {
    let (id_ranges_str, ids_str) = input.split_once("\n\n").unwrap();
    let mut no_ranges = NonOverlappingRanges::default();
    for id_range_str in id_ranges_str.lines() {
        let (start_str, end_str) = id_range_str.split_once('-').unwrap();
        let start = start_str.parse::<u64>().unwrap();
        let end = end_str.parse::<u64>().unwrap();
        no_ranges.insert(start..=end);
    }
    ids_str
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|id| no_ranges.contains(id))
        .count() as u64
}

fn part2(input: &str) -> u64 {
    let (id_ranges_str, _) = input.split_once("\n\n").unwrap();
    let mut no_ranges = NonOverlappingRanges::default();
    for id_range_str in id_ranges_str.lines() {
        let (start_str, end_str) = id_range_str.split_once('-').unwrap();
        let start = start_str.parse::<u64>().unwrap();
        let end = end_str.parse::<u64>().unwrap();
        no_ranges.insert(start..=end);
    }
    no_ranges.count()
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data")
        .join("day05.dat");
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

    const EXAMPLE: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 14);
    }
}
