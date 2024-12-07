use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../data/day6");
    let one = part1(input);
    println!("Part 1: {}", one);
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let (mut grid, mut guard) = parse(input);

    let mut total = 1; // One for the starting location

    loop {
        match guard.get_front(&grid) {
            Some(Point::Obstacle) => guard.turn(),
            Some(Point::Visited) => guard.forward(),
            Some(Point::Empty) => {
                guard.forward();
                total += 1;
                grid[guard.y][guard.x] = Point::Visited;
            }
            None => {
                break;
            }
        }
    }

    total
}

fn part2(input: &str) -> usize {
    let (mut grid, mut guard) = parse(input);
    let start = (guard.x, guard.y);

    let mut blocks = HashSet::new();

    loop {
        match guard.get_front(&grid) {
            Some(Point::Obstacle) => guard.turn(),
            Some(Point::Visited) => guard.forward(),
            Some(Point::Empty) => {
                guard.forward();
            }
            None => {
                break;
            }
        }

        if guard.get_front(&grid).is_some() {
            let (x, y) = guard.front_index();
            let previous = grid[y][x];
            grid[y][x] = Point::Obstacle;

            if guard.check_loop(&mut grid) {
                blocks.insert((x, y));
            }

            grid[y][x] = previous;
        }
    }

    blocks.remove(&start);
    blocks.len()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Point {
    Empty,
    Visited,
    Obstacle,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Guard {
    facing: Direction,
    x: usize,
    y: usize,
    turns: Vec<(Direction, usize, usize)>,
    path: Vec<(usize, usize)>, // Track the path
}

impl Guard {
    fn new(facing: Direction, x: usize, y: usize) -> Self {
        Guard {
            facing,
            x,
            y,
            turns: Vec::new(),
            path: vec![(x, y)], // Initialize with starting position
        }
    }

    fn front_index(&self) -> (usize, usize) {
        use Direction::*;
        match self.facing {
            Up => (self.x, self.y - 1),
            Down => (self.x, self.y + 1),
            Left => (self.x - 1, self.y),
            Right => (self.x + 1, self.y),
        }
    }

    fn get_front(&self, grid: &[Vec<Point>]) -> Option<Point> {
        use Direction::*;
        match self.facing {
            Up => {
                if self.y != 0 {
                    let (x, y) = self.front_index();
                    Some(grid[y][x])
                } else {
                    None
                }
            }
            Down => {
                if self.y < grid.len() - 1 {
                    let (x, y) = self.front_index();
                    Some(grid[y][x])
                } else {
                    None
                }
            }
            Left => {
                if self.x != 0 {
                    let (x, y) = self.front_index();
                    Some(grid[y][x])
                } else {
                    None
                }
            }
            Right => {
                if self.x < grid[self.y].len() - 1 {
                    let (x, y) = self.front_index();
                    Some(grid[y][x])
                } else {
                    None
                }
            }
        }
    }

    fn turn(&mut self) {
        use Direction::*;
        self.turns.push((self.facing, self.x, self.y));
        match self.facing {
            Up => self.facing = Right,
            Right => self.facing = Down,
            Down => self.facing = Left,
            Left => self.facing = Up,
        };
    }

    fn forward(&mut self) {
        use Direction::*;
        match self.facing {
            Up => self.y -= 1,
            Down => self.y += 1,
            Left => self.x -= 1,
            Right => self.x += 1,
        }

        self.path.push((self.x, self.y)); // Record the move
    }

    fn check_loop(&mut self, grid: &mut [Vec<Point>]) -> bool {
        // Check for loops
        let mut new_guard = Guard::new(self.facing, self.x, self.y);

        loop {
            match new_guard.get_front(grid) {
                Some(Point::Empty) => new_guard.forward(),
                Some(Point::Visited) => {
                    new_guard.forward();
                }
                Some(Point::Obstacle) => {
                    if new_guard
                        .turns
                        .contains(&(new_guard.facing, new_guard.x, new_guard.y))
                    {
                        //print_grid_with_path(grid, &new_guard.path);
                        return true;
                    }
                    new_guard.turn();
                }
                None => {
                    break;
                }
            }
        }

        false
    }
}

fn parse(input: &str) -> (Vec<Vec<Point>>, Guard) {
    let mut grid = vec![];
    let mut guard = None;
    let mut location = (0, 0);

    for line in input.lines() {
        location.1 += 1;
        location.0 = 0;
        let mut cur_line = vec![];
        for char in line.chars() {
            location.0 += 1;
            let point = match char {
                '.' => Point::Empty,
                '#' => Point::Obstacle,
                '^' => {
                    guard = Some(Guard::new(Direction::Up, location.0 - 1, location.1 - 1));
                    Point::Visited
                }
                _ => todo!(), // This shouldn't happen
            };
            cur_line.push(point);
        }
        grid.push(cur_line);
    }

    (grid, guard.unwrap())
}

fn print_grid_with_path(grid: &[Vec<Point>], path: &[(usize, usize)]) {
    let mut grid_copy = grid.to_vec();
    for &(x, y) in path {
        grid_copy[y][x] = Point::Visited; // Mark the path for visualization
    }
    for row in grid_copy {
        println!(
            "{}",
            row.iter()
                .map(|p| match p {
                    Point::Empty => '.',
                    Point::Visited => 'L', // 'L' for Loop path
                    Point::Obstacle => '#',
                })
                .collect::<String>()
        );
    }
    println!("---")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        let traveled = part1(INPUT);
        assert_eq!(traveled, 41);
    }

    #[test]
    fn test_part2() {
        let blocks = part2(INPUT);
        assert_eq!(blocks, 6);
    }
}
