use std::fs::read_to_string;

pub fn run() {
    let input = read_to_string("data/day2").unwrap();

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}

fn part1(input: String) -> i32 {
    let mut safe_count = 0;
    for line in input.lines() {
        let nums = parse_line(line);

        // Check if the line is safe
        if is_safe(nums) {
            safe_count += 1;
        }
    }

    safe_count
}

fn part2(input: String) -> i32 {
    let mut safe_count = 0;
    for line in input.lines() {
        let nums = parse_line(line);

        // If it's safe, increment the count and continue
        if is_safe(nums.clone()) {
            safe_count += 1;
            continue;
        }

        // If this line isn't safe, try removing each number and checking if the rest
        // is safe
        let mut all_nums = vec![];

        for i in 0..nums.len() {
            let mut new_nums = nums.clone();
            new_nums.remove(i);
            all_nums.push(new_nums);
        }

        for nums in all_nums {
            if is_safe(nums) {
                safe_count += 1;
                break;
            }
        }
    }

    safe_count
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn is_safe(nums: Vec<i32>) -> bool {
    let mut increasing = true;
    for (i, num) in nums.iter().enumerate() {
        if i == 0 {
            continue;
        } else {
            let diff = num - nums[i - 1];
            if diff.abs() < 1 || diff.abs() > 3 {
                return false;
            }
            if i == 1 {
                if num < &nums[0] {
                    increasing = false;
                }
            } else {
                match increasing {
                    true => {
                        if num < &nums[i - 1] {
                            return false;
                        }
                    }
                    false => {
                        if num > &nums[i - 1] {
                            return false;
                        }
                    }
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.to_string()), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_string()), 4);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("1 2 3 4 5"), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_index() {
        assert!(is_safe(vec![7, 6, 4, 2, 1]));
        assert!(!is_safe(vec![1, 3, 2, 4, 5]));
    }
}
