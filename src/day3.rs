use std::fs::read_to_string;

// Remember, we're doing this without regex, because what's the fun in that?
// Gotta write our own parser!
pub fn run() {
    let input = read_to_string("data/day3").unwrap();
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}

fn part1(input: String) -> i64 {
    let funcs = parse_muls(input);
    let mut total = 0;

    for (func_name, args) in funcs.clone() {
        if func_name == "mul" {
            let a: i64 = args[0].parse().unwrap();
            let b: i64 = args[1].parse().unwrap();
            total += a * b;
        }
    }

    total
}

fn part2(input: String) -> i64 {
    let funcs = parse_muls(input);
    let mut total = 0;

    let mut enabled = true;

    for (func_name, args) in funcs {
        if func_name == "mul" && enabled {
            let a: i64 = args[0].parse().unwrap();
            let b: i64 = args[1].parse().unwrap();
            total += a * b;
        } else if func_name == "do" {
            enabled = true;
        } else if func_name == "don't" {
            enabled = false;
        }
    }

    total
}

fn parse_muls(input: String) -> Vec<(String, Vec<String>)> {
    let mut chars = input.chars().peekable();

    let mut functions = vec![];

    while let Some(ch) = chars.next() {
        if ch == 'm' || ch == 'd' {
            // Parse the function name
            let mut func_name = ch.to_string();
            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_alphabetic() || next_ch == '\'' {
                    func_name.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            // Check if it's followed by parentheses
            if chars.peek() == Some(&'(') {
                chars.next(); // Consume '('

                // Parse the arguments
                let mut args = Vec::new();
                let mut current_arg = String::new();

                while let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        ',' => {
                            // Finish the current argument
                            if !current_arg.is_empty() {
                                args.push(current_arg.to_string());
                                current_arg = String::new();
                            }
                            chars.next(); // Consume ','
                        }
                        ')' => {
                            // Finish the last argument
                            if !current_arg.is_empty() {
                                args.push(current_arg.to_string());
                            }
                            chars.next(); // Consume ')'
                            break;
                        }
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            // Collect the argument
                            current_arg.push(next_ch);
                            chars.next();
                        }
                        _ => {
                            // Illegal character
                            chars.next();
                            break;
                        }
                    }
                }

                // Check if the function is legal
                if (args.is_empty() && (func_name == "do" || func_name == "don't"))
                    || (args.len() == 2 && func_name == "mul")
                {
                    functions.push((func_name, args));
                }
            }
        }
    }

    functions
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SECOND_INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1_parse() {
        let funcs = parse_muls(FIRST_INPUT.to_string());
        assert_eq!(funcs.len(), 4);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(FIRST_INPUT.to_string()), 161);
    }

    #[test]
    fn test_part2_parse() {
        let funcs = parse_muls(SECOND_INPUT.to_string());
        assert_eq!(funcs.len(), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SECOND_INPUT.to_string()), 48);
    }
}
