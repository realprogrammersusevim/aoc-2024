use std::fs::read_to_string;

pub fn run() {
    let input = read_to_string("data/day1").unwrap();
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}

fn parse_input(input: String) -> (Vec<i32>, Vec<i32>) {
    let mut first = vec![];
    let mut second = vec![];

    for line in input.lines() {
        let nums = line.trim().split("   ").collect::<Vec<&str>>();
        first.push(nums[0].parse::<i32>().unwrap());
        second.push(nums[1].parse::<i32>().unwrap());
    }

    (first, second)
}

fn part1(input: String) -> i32 {
    let (mut first, mut second) = parse_input(input);

    first.sort();
    second.sort();

    let mut distance = 0;
    for (i, num) in first.iter().enumerate() {
        distance += (num - second[i]).abs();
    }

    distance
}

fn part2(input: String) -> i32 {
    let (first, second) = parse_input(input);

    let mut similarity: usize = 0;
    for num in first {
        // find the number of times the number appears in the second list
        let count = second.iter().filter(|&x| *x == num).count();
        similarity += count * num as usize;
    }

    similarity as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!(part1(input.to_string()), 11);
    }

    #[test]
    fn test_part2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!(part2(input.to_string()), 31);
    }
}
