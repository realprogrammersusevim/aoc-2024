use std::collections::VecDeque;
use std::fs::read_to_string;

// Remember, we're doing this without regex, because what's the fun in that?
// Gotta write our own parser!
pub fn run() {
    let input = read_to_string("data/day3").unwrap();
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}

fn part1(input: String) -> i64 {
    let mut state = State::new(input.chars().collect(), false);
    state.execute();
    state.acc as i64
}

fn part2(input: String) -> i64 {
    let mut state = State::new(input.chars().collect(), true);
    state.execute();
    state.acc as i64
}

struct State {
    memory: VecDeque<char>,
    allow_disabled: bool,
    enabled: bool,
    acc: i32,
}

impl State {
    fn new(memory: VecDeque<char>, allow_disabled: bool) -> Self {
        Self {
            memory,
            allow_disabled,
            enabled: true,
            acc: 0,
        }
    }

    fn execute(&mut self) {
        loop {
            if self.memory.is_empty() {
                break;
            }

            match self.memory.front() {
                Some(&'d') => {
                    if let Some(((), remain)) = self.parse_static("do()") {
                        self.state(true);
                        self.memory = remain;
                        continue;
                    } else if let Some(((), remain)) = self.parse_static("don't()") {
                        self.state(false);
                        self.memory = remain;
                        continue;
                    }
                }
                Some(&'m') => {
                    if let Some(((x, y), remain)) = self.parse_mul() {
                        if self.enabled {
                            self.acc += x * y;
                        }
                        self.memory = remain;
                        continue;
                    }
                }
                _ => {}
            }

            self.memory.pop_front(); // Drop the first character
        }
    }

    fn state(&mut self, state: bool) {
        if self.allow_disabled {
            self.enabled = state;
        }
    }

    fn parse_static(&self, prefix: &str) -> Option<((), VecDeque<char>)> {
        let mut memory = self.memory.clone();
        if memory.len() >= prefix.len()
            && memory.iter().take(prefix.len()).collect::<String>() == prefix
        {
            for _ in 0..prefix.len() {
                memory.pop_front();
            }
            Some(((), memory))
        } else {
            None
        }
    }

    fn parse_mul(&self) -> Option<((i32, i32), VecDeque<char>)> {
        let mut memory = self.memory.clone();
        let prefix = "mul(";
        if memory.len() < prefix.len()
            || memory.iter().take(prefix.len()).collect::<String>() != prefix
        {
            return None;
        }

        for _ in 0..prefix.len() {
            memory.pop_front(); // Remove "mul("
        }

        let mut num_buf = String::new();
        while let Some(c) = memory.front() {
            if *c == ',' {
                memory.pop_front(); // Remove ','
                break;
            } else if c.is_ascii_digit() {
                num_buf.push(*c);
                memory.pop_front();
            } else {
                return None; // Invalid character for number parsing
            }
        }

        let x: i32 = num_buf.parse().ok()?;
        num_buf.clear();

        while let Some(c) = memory.front() {
            if *c == ')' {
                memory.pop_front(); // Remove ')'
                break;
            } else if c.is_ascii_digit() || *c == '-' {
                num_buf.push(*c);
                memory.pop_front();
            } else {
                return None; // Invalid character for number parsing
            }
        }

        let y: i32 = num_buf.parse().ok()?;
        Some(((x, y), memory))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SECOND_INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        let initial_memory: VecDeque<char> = FIRST_INPUT.chars().collect();
        let mut state = State::new(initial_memory, false);
        state.execute();
        //assert_eq!(state.enabled, true);
        assert_eq!(state.acc, 161);
    }

    #[test]
    fn test_part2() {
        let initial_memory: VecDeque<char> = SECOND_INPUT.chars().collect();
        let mut state = State::new(initial_memory, true);
        state.execute();
        //assert_eq!(state.enabled, true);
        assert_eq!(state.acc, 48);
    }
}
