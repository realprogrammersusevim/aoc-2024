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
    let mut smallers: Vec<usize> = vec![];
    let mut largers: Vec<usize> = vec![];

    for (smaller, larger) in &rules {
        smallers.push(*smaller);
        largers.push(*larger);
    }

    smallers.sort();
    smallers.dedup();
    largers.sort();
    largers.dedup();

    let smallest: Vec<&usize> = smallers.iter().filter(|x| !largers.contains(x)).collect();

    let mut sorted = vec![*smallest[0]];

    // Only add the number if all the rules where it's larger are when the number is contained
    // in sorted
    while !largers.is_empty() {
        let mut should_add: (usize, usize) = (0, 0);
        for (i, num) in largers.iter().enumerate() {
            let new_rules = rules.clone();
            let all_smallers: Vec<usize> = new_rules
                .iter()
                .filter(|(_, larger)| larger == num)
                .map(|(smaller, _)| *smaller)
                .collect();

            let mut is_next = true;

            for smaller in all_smallers {
                if !sorted.contains(&(smaller)) {
                    is_next = false;
                    break;
                }
            }

            if is_next {
                should_add = (i, *num);
            }
        }

        largers.swap_remove(should_add.0);
        sorted.push(should_add.1);
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
