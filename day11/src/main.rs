use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
    time::Instant,
};

struct Graph<'a> {
    edges: HashMap<&'a str, Vec<&'a str>>,
    sorted: Vec<&'a str>,
}

impl<'a> Graph<'a> {
    fn parse(input: &'a str) -> Self {
        let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut all_nodes: Vec<&str> = Vec::new();
        for line in input.trim().lines() {
            let (id_in, out_str) = line.split_once(": ").unwrap();
            let targets: Vec<&str> = out_str.split_whitespace().collect();
            edges.insert(id_in, targets);
            if !all_nodes.contains(&id_in) {
                all_nodes.push(id_in);
            }
            for t in out_str.split_whitespace() {
                if !all_nodes.contains(&t) {
                    all_nodes.push(t);
                }
            }
        }

        let sorted = Self::topological_sort(&edges, &all_nodes);
        Self { edges, sorted }
    }

    fn topological_sort(
        edges: &HashMap<&'a str, Vec<&'a str>>,
        all_nodes: &[&'a str],
    ) -> Vec<&'a str> {
        let mut in_degree: HashMap<&str, usize> = HashMap::new();
        for &node in all_nodes {
            in_degree.insert(node, 0);
        }
        for targets in edges.values() {
            for &t in targets {
                *in_degree.get_mut(t).unwrap() += 1;
            }
        }

        let mut queue = VecDeque::new();
        for &node in all_nodes {
            if in_degree[node] == 0 {
                queue.push_back(node);
            }
        }

        let mut sorted: Vec<&str> = Vec::new();
        while let Some(node) = queue.pop_front() {
            sorted.push(node);
            if let Some(targets) = edges.get(node) {
                for &t in targets {
                    *in_degree.get_mut(t).unwrap() -= 1;
                    if in_degree[t] == 0 {
                        queue.push_back(t);
                    }
                }
            }
        }
        sorted
    }

    fn count_paths(&self, from: &str, to: &str) -> u64 {
        let i_from = self.sorted.iter().position(|&n| n == from).unwrap();
        let i_to = self.sorted.iter().position(|&n| n == to).unwrap();
        let mut c: Vec<u64> = vec![0; self.sorted.len()];
        c[i_from] = 1;
        for i in (i_from + 1)..=i_to {
            for j in i_from..i {
                if let Some(targets) = self.edges.get(self.sorted[j])
                    && targets.contains(&self.sorted[i])
                {
                    c[i] += c[j];
                }
            }
        }
        c[i_to]
    }
}

fn part1(input: &str) -> u64 {
    let graph = Graph::parse(input);
    graph.count_paths("you", "out")
}

fn part2(input: &str) -> u64 {
    let graph = Graph::parse(input);
    let svr_dac = graph.count_paths("svr", "dac");
    let dac_fft = graph.count_paths("dac", "fft");
    let fft_out = graph.count_paths("fft", "out");
    let svr_fft = graph.count_paths("svr", "fft");
    let fft_dac = graph.count_paths("fft", "dac");
    let dac_out = graph.count_paths("dac", "out");
    svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data")
        .join("day11.dat");
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

    const EXAMPLE_P1: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_P1), 5);
    }

    const EXAMPLE_P2: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_P2), 2);
    }
}
