use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

fn bron_kerbosch(
    adj: &HashMap<String, HashSet<String>>,
    r: &mut HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    max_clique: &mut Vec<HashSet<String>>,
) {
    if p.len() == 0 && x.len() == 0 {
        if r.len() > max_clique[0].len() {
            max_clique[0] = r.clone()
        }
        return;
    }

    let pivot = p
        .union(x)
        .map(|u| (u, p.intersection(&adj[u]).count()))
        .max_by(|(_, a), (_, b)| a.cmp(b));

    let Some((pivot, _)) = pivot else { return };

    for v in p.clone().difference(&adj[pivot]) {
        let mut r_union = HashSet::new();
        r_union.insert(v.clone());

        let mut new_r = r.union(&r_union).cloned().collect();
        let mut new_p = p.intersection(&adj[v]).cloned().collect();
        let mut new_x = x.intersection(&adj[v]).cloned().collect();
        bron_kerbosch(adj, &mut new_r, &mut new_p, &mut new_x, max_clique);
        p.remove(v);
        x.insert(v.clone());
    }
}

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

    fn get_biconnected_adj(&self) -> HashMap<String, HashSet<String>> {
        let mut adjecency_list: HashMap<String, HashSet<String>> = HashMap::new();

        for (a, b) in &self.edges {
            adjecency_list
                .entry(a.clone())
                .or_default()
                .insert(b.clone());
            adjecency_list
                .entry(b.clone())
                .or_default()
                .insert(a.clone());
        }

        adjecency_list
    }

    fn find_groups(&self) -> HashSet<[String; 3]> {
        let adj = self.get_biconnected_adj();

        let nodes: Vec<&String> = adj.keys().collect();

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
                let a_neighbors = adj.get(*a).unwrap();
                let b_neigbors = adj.get(*b).unwrap();
                let c_neigbors = adj.get(*c).unwrap();

                a_neighbors.contains(*b)
                    && a_neighbors.contains(*c)
                    && b_neigbors.contains(*a)
                    && b_neigbors.contains(*c)
                    && c_neigbors.contains(*a)
                    && c_neigbors.contains(*b)
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

    fn maximum_clique(&self) -> HashSet<String> {
        let adj = self.get_biconnected_adj();

        let mut r = HashSet::new();
        let mut p = adj.keys().cloned().collect();
        let mut x = HashSet::new();
        let mut max_clique = vec![HashSet::new()];

        bron_kerbosch(&adj, &mut r, &mut p, &mut x, &mut max_clique);

        return max_clique.first().unwrap().clone();
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

    let maximum_clique = g
        .maximum_clique()
        .into_iter()
        .sorted_by(|a, b| a.cmp(b))
        .collect_vec();

    let result = maximum_clique.join(",");

    println!("Result (Part 2): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
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

    #[test]
    fn test_find_groups() {
        let g = Graph::parse(&INPUT);

        let groups = g.find_groups();

        assert_eq!(groups.len(), 12);
    }

    #[test]
    fn test_find_max_clique() {
        let g = Graph::parse(&INPUT);

        assert_eq!(
            g.maximum_clique(),
            [
                String::from("co"),
                String::from("de"),
                String::from("ka"),
                String::from("ta"),
            ]
            .into_iter()
            .collect()
        );
    }
}
