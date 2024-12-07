pub fn run() {
    let input = include_str!("../data/day7");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let parsed = parse(input);

    let mut total = 0;

    for calib in parsed {
        let mut combinations = vec![vec![]];

        for _ in 0..calib.1.len() - 1 {
            let mut new_combinations = Vec::with_capacity(combinations.len() * 2);
            for combination in &combinations {
                let mut with_add = combination.clone();
                with_add.push(Operation::Add);
                new_combinations.push(with_add);

                let mut with_mul = combination.clone();
                with_mul.push(Operation::Mul);
                new_combinations.push(with_mul);
            }
            combinations = new_combinations;
        }
        for op_list in combinations {
            let mut val = calib.1[0];
            for (i, op) in op_list.iter().enumerate() {
                match op {
                    Operation::Add => val += calib.1[i + 1],
                    Operation::Mul => val *= calib.1[i + 1],
                    Operation::Concat => panic!(),
                }
            }

            if val == calib.0 {
                total += val;
                break; // Multiple operation combos might match the total
            }
        }
    }

    total
}

fn part2(input: &str) -> usize {
    let parsed = parse(input);

    let mut total = 0;

    for calib in parsed {
        let mut combinations = vec![vec![]];

        for _ in 0..calib.1.len() - 1 {
            let mut new_combinations = Vec::with_capacity(combinations.len() * 2);
            for combination in &combinations {
                let mut with_add = combination.clone();
                with_add.push(Operation::Add);
                new_combinations.push(with_add);

                let mut with_mul = combination.clone();
                with_mul.push(Operation::Mul);
                new_combinations.push(with_mul);

                let mut with_concat = combination.clone();
                with_concat.push(Operation::Concat);
                new_combinations.push(with_concat);
            }
            combinations = new_combinations;
        }
        for op_list in combinations {
            let mut val = calib.1[0];
            for (i, op) in op_list.iter().enumerate() {
                match op {
                    Operation::Add => val += calib.1[i + 1],
                    Operation::Mul => val *= calib.1[i + 1],
                    Operation::Concat => val = concat(val, calib.1[i + 1]),
                }
            }

            if val == calib.0 {
                total += val;
                break; // Multiple operation combos might match the total
            }
        }
    }

    total
}

fn concat(a: usize, b: usize) -> usize {
    let mut multiplier = 1;
    let mut temp = b;

    while temp > 0 {
        multiplier *= 10;
        temp /= 10;
    }

    a * multiplier + b
}

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Mul,
    Concat,
}

fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    let mut parsed = Vec::new();
    for line in input.lines() {
        let str: Vec<&str> = line.split(": ").collect();
        let test: usize = str[0].parse().expect("Couldn't parse test value as number");
        let nums: Vec<usize> = str[1]
            .split(" ")
            .map(|x| x.parse().expect("Couldn't parse number"))
            .collect();
        parsed.push((test, nums))
    }

    parsed
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 11387);
    }
}
