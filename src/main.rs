use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

impl TryFrom<&str> for Op {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "OR" => Op::Or,
            "AND" => Op::And,
            "XOR" => Op::Xor,
            _ => return Err(()),
        })
    }
}

struct System {
    node_values: HashMap<String, bool>,
    nodes: HashSet<String>,
    edges: HashSet<(String, Op, String, String)>,
}

impl System {
    fn parse(input: &str) -> Self {
        let input = input.trim();

        let mut parts = input.split("\n\n");

        let initial_value_input = parts.next().unwrap();
        let connections_input = parts.next().unwrap();

        let node_values = initial_value_input
            .lines()
            .map(|line| {
                let line = line.trim();
                let mut parts = line.split(": ");
                let node = parts.next().unwrap().to_string();
                let value = parts.next().unwrap() == "1";
                (node, value)
            })
            .collect();

        let mut nodes = HashSet::new();
        let mut edges = HashSet::new();

        for line in connections_input.lines() {
            let line = line.trim();
            let mut split = line.split(" -> ");
            let left = split.next().unwrap();
            let out = split.next().unwrap().to_string();
            let mut split = left.split(" ");
            let i1 = split.next().unwrap().to_string();
            let op = Op::try_from(split.next().unwrap()).unwrap();
            let i2 = split.next().unwrap().to_string();
            nodes.insert(out.clone());
            nodes.insert(i1.clone());
            nodes.insert(i2.clone());
            edges.insert((i1, op, i2, out));
        }

        Self {
            node_values,
            nodes,
            edges,
        }
    }

    fn solve_node(&mut self, node: &str) {
        if *(&self.node_values.contains_key(node)) {
            return;
        }

        let edges = self.edges.clone();

        let Some((i1, op, i2, _)) = edges.iter().find(|(_, _, _, x)| x == node) else {
            panic!("Node {node} does not have an input edge");
        };

        if !&self.node_values.contains_key(i1) {
            self.solve_node(i1);
        }

        if !&self.node_values.contains_key(i2) {
            self.solve_node(i2);
        }

        let i1v = self.node_values[i1];
        let i2v = self.node_values[i2];

        let v = match op {
            Op::And => i1v && i2v,
            Op::Or => i1v || i2v,
            Op::Xor => i1v != i2v,
        };

        self.node_values.insert(node.to_string(), v);
    }

    fn solve(&mut self) -> isize {
        let mut nodes = self.nodes.clone().into_iter().collect_vec();

        nodes.sort();

        let res: Vec<bool> = nodes
            .into_iter()
            .filter(|n| n.starts_with("z"))
            .map(|n| {
                self.solve_node(&n);
                self.node_values[&n]
            })
            .rev()
            .collect();

        let res = isize::from_str_radix(
            &res.into_iter()
                .map(|v| match v {
                    true => '1',
                    false => '0',
                })
                .join(""),
            2,
        )
        .unwrap();

        res
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/day24.txt").expect("Failed to read file");

    let mut s = System::parse(&input);

    let result = s.solve();

    println!("Result (Part 1): {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let input = r#"
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
        "#;

        let mut s = System::parse(&input);

        let result = s.solve();

        assert_eq!(
            s.node_values,
            [
                ("x00".to_string(), true),
                ("x01".to_string(), true),
                ("x02".to_string(), true),
                ("y00".to_string(), false),
                ("y01".to_string(), true),
                ("y02".to_string(), false),
                ("z00".to_string(), false),
                ("z01".to_string(), false),
                ("z02".to_string(), true),
            ]
            .into_iter()
            .collect()
        );

        assert_eq!(result, 4);
    }

    #[test]
    fn test_example_two() {
        let input = r#"
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
        "#;

        let mut s = System::parse(&input);

        assert_eq!(s.solve(), 2024);
    }
}
