use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use nalgebra::Vector3;
use ndarray::Array2;

fn parse_input(input: &str) -> Vec<Vector3<f64>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line
                .trim()
                .split(',')
                .map(|part| part.trim().parse().unwrap());
            Vector3::new(
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        })
        .collect()
}

fn compute_distance_matrix(points: &[Vector3<f64>]) -> Array2<f64> {
    let n = points.len();
    let mut dist_matrix = Array2::zeros((n, n));
    for i in 0..n {
        for j in i + 1..n {
            let dist = (points[i] - points[j]).norm();
            dist_matrix[[i, j]] = dist;
            dist_matrix[[j, i]] = f64::INFINITY;
        }
        dist_matrix[[i, i]] = f64::INFINITY;
    }
    dist_matrix
}

fn part1(input: &str, n_junc: usize) -> u64 {
    let points = parse_input(input);
    let mut dist_matrix = compute_distance_matrix(&points);
    let mut circuits = HashMap::<usize, HashSet<usize>>::from_iter(
        (0..points.len()).map(|i| (i, HashSet::from_iter([i]))),
    );
    let mut map_to_circuit = HashMap::<usize, usize>::from_iter((0..points.len()).map(|i| (i, i)));
    let mut max_circ_id = points.len();
    for _ in 0..n_junc {
        let (i_min, j_min) = dist_matrix
            .indexed_iter()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        dist_matrix[[i_min, j_min]] = f64::INFINITY;
        let circ_i_id = *map_to_circuit.get(&i_min).unwrap();
        let circ_j_id = *map_to_circuit.get(&j_min).unwrap();
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
    let mut dist_matrix = compute_distance_matrix(&points);
    let mut circuits = HashMap::<usize, HashSet<usize>>::from_iter(
        (0..points.len()).map(|i| (i, HashSet::from_iter([i]))),
    );
    let mut map_to_circuit = HashMap::<usize, usize>::from_iter((0..points.len()).map(|i| (i, i)));
    let mut max_circ_id = points.len();
    loop {
        let (i_min, j_min) = dist_matrix
            .indexed_iter()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        dist_matrix[[i_min, j_min]] = f64::INFINITY;
        let circ_i_id = *map_to_circuit.get(&i_min).unwrap();
        let circ_j_id = *map_to_circuit.get(&j_min).unwrap();
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
            return points[i_min].x as u64 * points[j_min].x as u64;
        }
    }
}

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../data/day08.dat"));
    let t = Instant::now();
    println!("Part 1: {}", part1(input, 1000));
    println!("Part 1 took: {:?}", t.elapsed());
    let t = Instant::now();
    println!("Part 2: {}", part2(input));
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
