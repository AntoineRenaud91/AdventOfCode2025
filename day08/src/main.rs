use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    time::Instant,
};

fn parse_input(input: &str) -> Vec<[i64; 3]> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line
                .trim()
                .split(',')
                .map(|part| part.trim().parse().unwrap());
            [
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            ]
        })
        .collect()
}

fn sorted_pair_indices(points: &[[i64; 3]]) -> impl Iterator<Item = [usize; 2]> {
    let mut indices = (0..points.len())
        .flat_map(|i| (i + 1..points.len()).map(move |j| [i, j]))
        .map(|[i, j]| {
            (
                [i, j],
                (points[i][0] - points[j][0]).pow(2)
                    + (points[i][1] - points[j][1]).pow(2)
                    + (points[i][2] - points[j][2]).pow(2),
            )
        })
        .collect::<Vec<_>>();
    indices.sort_by_key(|(_, d)| *d);
    indices.into_iter().map(|(ind, _)| ind)
}

fn part1(input: &str, n_junc: usize) -> u64 {
    let points = parse_input(input);
    let mut circuits = HashMap::<usize, HashSet<usize>>::from_iter(
        (0..points.len()).map(|i| (i, HashSet::from_iter([i]))),
    );
    let mut map_to_circuit = HashMap::<usize, usize>::from_iter((0..points.len()).map(|i| (i, i)));
    let mut max_circ_id = points.len();
    for [i, j] in sorted_pair_indices(&points).take(n_junc) {
        let circ_i_id = *map_to_circuit.get(&i).unwrap();
        let circ_j_id = *map_to_circuit.get(&j).unwrap();
        if circ_i_id == circ_j_id {
            continue;
        }
        let mut new_circ = circuits.remove(&circ_i_id).unwrap();
        new_circ.extend(circuits.remove(&circ_j_id).unwrap());
        for i in &new_circ {
            map_to_circuit.insert(*i, max_circ_id);
        }
        circuits.insert(max_circ_id, new_circ);
        max_circ_id += 1;
    }
    let mut circuit_size = circuits
        .values()
        .map(|circ| circ.len() as u64)
        .collect::<Vec<u64>>();
    circuit_size.sort();
    circuit_size.reverse();
    circuit_size.into_iter().take(3).product()
}

fn part2(input: &str) -> u64 {
    let points = parse_input(input);
    let mut circuits = HashMap::<usize, HashSet<usize>>::from_iter(
        (0..points.len()).map(|i| (i, HashSet::from_iter([i]))),
    );
    let mut map_to_circuit = HashMap::<usize, usize>::from_iter((0..points.len()).map(|i| (i, i)));
    let mut max_circ_id = points.len();
    for [i, j] in sorted_pair_indices(&points) {
        let circ_i_id = *map_to_circuit.get(&i).unwrap();
        let circ_j_id = *map_to_circuit.get(&j).unwrap();
        if circ_i_id == circ_j_id {
            continue;
        }
        let mut new_circ = circuits.remove(&circ_i_id).unwrap();
        new_circ.extend(circuits.remove(&circ_j_id).unwrap());
        for i in &new_circ {
            map_to_circuit.insert(*i, max_circ_id);
        }
        circuits.insert(max_circ_id, new_circ);
        max_circ_id += 1;
        if circuits.len() == 1 {
            return (points[i][0] * points[j][0]) as u64;
        }
    }
    unreachable!()
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data")
        .join("day08.dat");
    let input = std::fs::read_to_string(path).unwrap();
    let t = Instant::now();
    println!("Part 1: {}", part1(&input, 1000));
    println!("Part 1 took: {:?}", t.elapsed());
    let t = Instant::now();
    println!("Part 2: {}", part2(&input));
    println!("Part 2 took: {:?}", t.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE, 10), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 25272);
    }
}
