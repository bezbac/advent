use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

struct Graph {
    edges: Vec<(String, String)>,
}

impl Graph {
    fn parse(input: &str) -> Self {
        let edges: Vec<(String, String)> = input
            .trim()
            .lines()
            .map(|line| {
                let mut parts = line.trim().split('-');
                let a = parts.next().unwrap().to_string();
                let b = parts.next().unwrap().to_string();
                (a, b)
            })
            .collect();

        Self { edges }
    }

    fn find_groups(&self) -> HashSet<[String; 3]> {
        let nodes: Vec<&String> = self
            .edges
            .iter()
            .flat_map(|(a, b)| [a, b])
            .unique()
            .collect();

        let mut adjecency_list: HashMap<&String, HashSet<&String>> = HashMap::new();

        for (a, b) in &self.edges {
            adjecency_list.entry(a).or_default().insert(b);
            adjecency_list.entry(b).or_default().insert(a);
        }

        let groups: HashSet<[String; 3]> = nodes
            .iter()
            .cartesian_product(nodes.iter())
            .filter_map(|(a, b)| {
                if a == b {
                    return None;
                }

                Some((a, b))
            })
            .cartesian_product(nodes.iter())
            .par_bridge()
            .filter_map(|((a, b), c)| {
                if a == c || b == c {
                    return None;
                }

                Some((a, b, c))
            })
            .filter(|&(a, b, c)| {
                let a_neighbors = adjecency_list.get(a).unwrap();
                let b_neigbors = adjecency_list.get(b).unwrap();
                let c_neigbors = adjecency_list.get(c).unwrap();

                a_neighbors.contains(b)
                    && a_neighbors.contains(c)
                    && b_neigbors.contains(a)
                    && b_neigbors.contains(c)
                    && c_neigbors.contains(a)
                    && c_neigbors.contains(b)
            })
            .map(|(a, b, c)| {
                dbg!(a);

                let a = (*a).clone();
                let b = (*b).clone();
                let c = (*c).clone();

                let mut group = [a, b, c];
                group.sort();
                group
            })
            .collect();

        groups.into_iter().dedup().collect()
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/day23.txt").expect("Failed to read file");

    let g = Graph::parse(&input);

    let groups = g.find_groups();

    let result = groups
        .par_iter()
        .filter(|nodes| nodes.iter().any(|node| node.starts_with('t')))
        .count();

    println!("Result (Part 1): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_groups() {
        let input = r#"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
        "#;

        let g = Graph::parse(&input);

        let groups = g.find_groups();

        assert_eq!(groups.len(), 12);
    }
}
