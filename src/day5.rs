use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../data/day5");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let (rules, updates) = parse(input);

    let mut total = 0;

    for update in updates {
        let update_rules = get_rules(update.clone(), rules.clone());
        let sorted_update = sort_by_rules(update_rules);
        let correct_update: Vec<usize> = sorted_update
            .iter()
            .copied()
            .filter(|x| update.contains(x))
            .collect();
        if correct_update == update {
            total += update[(update.len()) / 2] // It's correct, get the middle
        }
    }

    total
}

fn part2(input: &str) -> usize {
    let (rules, updates) = parse(input);

    let mut total = 0;

    for update in updates {
        let update_rules = get_rules(update.clone(), rules.clone());
        let sorted_update = sort_by_rules(update_rules);
        let correct_update: Vec<usize> = sorted_update
            .iter()
            .copied()
            .filter(|x| update.contains(x))
            .collect();

        if correct_update != update {
            total += correct_update[(correct_update.len()) / 2];
        }
    }

    total
}

fn get_rules(update: Vec<usize>, rules: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    rules
        .iter()
        .copied()
        .filter(|(smaller, larger)| update.contains(smaller) && update.contains(larger))
        .collect()
}

fn sort_by_rules(rules: Vec<(usize, usize)>) -> Vec<usize> {
    // Create a graph representation
    let mut in_degree = HashMap::new(); // Tracks how many prerequisites each node has
    let mut graph = HashMap::new(); // Adjacency list

    for (smaller, larger) in rules {
        graph.entry(smaller).or_insert_with(Vec::new).push(larger);
        *in_degree.entry(larger).or_insert(0) += 1;
        in_degree.entry(smaller).or_insert(0); // Ensure smaller exists in the in-degree map
    }

    // Find all items with no prerequisites (in-degree = 0)
    let mut queue: Vec<usize> = in_degree
        .iter()
        .filter(|(_, &degree)| degree == 0)
        .map(|(&node, _)| node)
        .collect();

    queue.sort(); // Ensure deterministic order for nodes with no prerequisites

    let mut sorted = Vec::new();

    // Topological sort using Kahn's algorithm
    while let Some(node) = queue.pop() {
        sorted.push(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                let entry = in_degree.get_mut(&neighbor).unwrap();
                *entry -= 1;
                if *entry == 0 {
                    queue.push(neighbor);
                }
            }
        }

        queue.sort(); // Keep queue sorted for deterministic output
    }

    sorted
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
        let (rules, _) = parse(INPUT);
        assert_eq!(sort_by_rules(rules), vec![97, 75, 47, 61, 53, 29, 13])
    }
}
