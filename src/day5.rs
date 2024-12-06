pub fn run() {
    let input = include_str!("../data/day5");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let (rules, updates) = parse(input);

    let mut total = 0;

    for update in updates {
        let sorted_update = sort_by_rules(update.clone(), rules.clone());
        if sorted_update == update {
            total += update[(update.len()) / 2] // It's correct, get the middle
        }
    }

    total
}

fn part2(input: &str) -> usize {
    let (rules, updates) = parse(input);

    let mut total = 0;

    for update in updates {
        let sorted_update = sort_by_rules(update.clone(), rules.clone());
        if sorted_update != update {
            total += sorted_update[(sorted_update.len()) / 2];
        }
    }

    total
}

pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

pub type NodeIndex = usize;

pub struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
}

pub type EdgeIndex = usize;

pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

impl Graph {
    pub fn add_node(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            first_outgoing_edge: None,
        });
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors {
            graph: self,
            current_edge_index: first_outgoing_edge,
        }
    }
}

pub struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

fn sort_by_rules(update: Vec<usize>, rules: Vec<(usize, usize)>) -> Vec<usize> {
    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    // Map from node value to its index in the graph
    let mut node_indices = std::collections::HashMap::new();

    // Add nodes and edges to the graph
    for (smaller, larger) in rules {
        if update.contains(&smaller) && update.contains(&larger) {
            let smaller_idx = *node_indices
                .entry(smaller)
                .or_insert_with(|| graph.add_node());
            let larger_idx = *node_indices
                .entry(larger)
                .or_insert_with(|| graph.add_node());
            graph.add_edge(smaller_idx, larger_idx);
        }
    }

    // Track in-degrees of all nodes
    let mut in_degree = vec![0; graph.nodes.len()];
    for node_idx in 0..graph.nodes.len() {
        for successor in graph.successors(node_idx) {
            in_degree[successor] += 1;
        }
    }

    // Find all nodes with no prerequisites (in-degree = 0)
    let mut queue: Vec<usize> = in_degree
        .iter()
        .enumerate()
        .filter(|&(_, &degree)| degree == 0)
        .map(|(idx, _)| idx)
        .collect();

    queue.sort_unstable();

    let mut sorted = Vec::new();

    while let Some(node_idx) = queue.pop() {
        sorted.push(node_idx);

        for successor in graph.successors(node_idx) {
            in_degree[successor] -= 1;
            if in_degree[successor] == 0 {
                queue.push(successor);
                queue.sort_unstable(); // Keep sorted for deterministic output
            }
        }
    }

    // Convert sorted indices back to original node values
    let mut index_to_value: Vec<_> = vec![0; graph.nodes.len()];
    for (value, &idx) in &node_indices {
        index_to_value[idx] = *value;
    }

    sorted
        .into_iter()
        .map(|idx| index_to_value[idx])
        .filter(|x| update.contains(x))
        .collect()
}

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut rules = vec![];
    let mut updates = vec![];

    let mut first = true;
    for line in input.lines() {
        if line.is_empty() {
            first = false;
            continue;
        }

        match first {
            true => {
                let split: Vec<usize> = line.split("|").map(|x| x.parse().unwrap()).collect();
                rules.push((split[0], split[1]));
            }
            false => {
                let split: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();
                updates.push(split);
            }
        }
    }

    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 123);
    }

    #[test]
    fn test_rule_sort() {
        let (rules, updates) = parse(INPUT);
        assert_eq!(
            sort_by_rules(updates[1].clone(), rules),
            vec![97, 61, 53, 29, 13]
        )
    }
}
