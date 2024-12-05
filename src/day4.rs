pub fn run() {
    let input = include_str!("../data/day4");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);

    let mut count = 0;

    let x_len = grid[0].len();
    for y in 0..grid.len() {
        for x in 0..x_len {
            if grid[y][x] == 'X' {
                // Check all directions individually
                if check_up(&grid, x, y) {
                    count += 1;
                }
                if check_down(&grid, x, y) {
                    count += 1;
                }
                if check_left(&grid, x, y) {
                    count += 1;
                }
                if check_right(&grid, x, y) {
                    count += 1;
                }
                if check_up_left(&grid, x, y) {
                    count += 1;
                }
                if check_up_right(&grid, x, y) {
                    count += 1;
                }
                if check_down_left(&grid, x, y) {
                    count += 1;
                }
                if check_down_right(&grid, x, y) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let mut count = 0;

    let x_len = grid[0].len();
    for y in 0..grid.len() {
        for x in 0..x_len {
            if grid[y][x] == 'A' && tl_br(&grid, x, y) && tr_bl(&grid, x, y) {
                count += 1;
            }
        }
    }

    count
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn check_up(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if y < 3 {
        return false; // XMAS is 4 characters long
    }

    for i in 1..=3 {
        match (i, grid[y - i][x]) {
            (1, 'M') | (2, 'A') => {} // All is well
            (3, 'S') => return true,  // Found XMAS
            _ => return false,        // Invalid character
        }
    }

    false
}

fn check_down(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if y + 3 >= grid.len() {
        return false; // XMAS is 4 characters long
    }

    for i in 1..=3 {
        match (i, grid[y + i][x]) {
            (1, 'M') | (2, 'A') => {} // All is well
            (3, 'S') => return true,  // Found XMAS
            _ => return false,        // Invalid character
        }
    }

    false
}

fn check_left(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x < 3 {
        return false; // XMAS is 4 characters long
    }

    for i in 1..=3 {
        match (i, grid[y][x - i]) {
            (1, 'M') | (2, 'A') => {} // All is well
            (3, 'S') => return true,  // Found XMAS
            _ => return false,        // Invalid character
        }
    }

    false
}

fn check_right(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x + 3 >= grid[y].len() {
        return false; // XMAS is 4 characters long
    }

    for i in 1..=3 {
        match (i, grid[y][x + i]) {
            (1, 'M') | (2, 'A') => {} // All is well
            (3, 'S') => return true,  // Found XMAS
            _ => return false,        // Invalid character
        }
    }

    false
}

fn check_up_left(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x < 3 || y < 3 {
        return false; // XMAS is 4 characters long
    }

    for i in 1..=3 {
        match (i, grid[y - i][x - i]) {
            (1, 'M') | (2, 'A') => {} // All is well
            (3, 'S') => return true,  // Found XMAS
            _ => return false,        // Invalid character
        }
    }

    false
}

fn check_up_right(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x + 3 >= grid[y].len() || y < 3 {
        return false; // XMAS is 4 characters long
    }

    for i in 1..=3 {
        match (i, grid[y - i][x + i]) {
            (1, 'M') | (2, 'A') => {} // All is well
            (3, 'S') => return true,  // Found XMAS
            _ => return false,        // Invalid character
        }
    }

    false
}

fn check_down_left(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x < 3 || y + 3 >= grid.len() {
        return false; // XMAS is 4 characters long
    }

    for i in 1..=3 {
        match (i, grid[y + i][x - i]) {
            (1, 'M') | (2, 'A') => {} // All is well
            (3, 'S') => return true,  // Found XMAS
            _ => return false,        // Invalid character
        }
    }

    false
}

fn check_down_right(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x + 3 >= grid[y].len() || y + 3 >= grid.len() {
        return false; // XMAS is 4 characters long
    }

    for i in 1..=3 {
        match (i, grid[y + i][x + i]) {
            (1, 'M') | (2, 'A') => {} // All is well
            (3, 'S') => return true,  // Found XMAS
            _ => return false,        // Invalid character
        }
    }

    false
}

fn tl_br(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || x + 1 >= grid[y].len() || y + 1 >= grid.len() {
        return false;
    }

    if (grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S')
        || (grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M')
    {
        return true;
    }

    false
}

fn tr_bl(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || x + 1 >= grid[y].len() || y + 1 >= grid.len() {
        return false;
    }

    if (grid[y - 1][x + 1] == 'M' && grid[y + 1][x - 1] == 'S')
        || (grid[y - 1][x + 1] == 'S' && grid[y + 1][x - 1] == 'M')
    {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 9);
    }
}
