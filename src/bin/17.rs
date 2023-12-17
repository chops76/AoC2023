use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: i64,
    x: usize,
    y: usize,
    last_dir: Dir,
    line_length: usize
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input.split("\n").map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i64).collect::<Vec<i64>>()).collect()    
}

fn solve(grid: &Vec<Vec<i64>>, min: usize, max: usize) -> Option<i64> {
    let mut heap = BinaryHeap::new();
    let mut hs = HashSet::new();

    heap.push(State { cost: 0, x: 0, y: 0, last_dir: Dir::Right, line_length: 0 });
    while let Some(State { cost, x, y, last_dir, line_length }) = heap.pop() {
        if hs.contains(&(x, y, last_dir, line_length)) {
            continue;
        }
        hs.insert((x, y, last_dir, line_length));
        if x == grid[0].len() - 1 && y == grid.len() - 1 && line_length >= min {
            return Some(cost);
        }
        match last_dir {
            Dir::Right => {
                if line_length != max && x != grid[0].len() - 1 {
                    heap.push(State { cost: cost + grid[y][x+1], x: x + 1, y: y, last_dir: Dir::Right, line_length: line_length + 1 });                    
                }
                if line_length >= min && y != 0 {
                    heap.push(State { cost: cost + grid[y-1][x], x: x, y: y - 1, last_dir: Dir::Up, line_length: 1 });                    
                }
                if line_length >= min && y != grid.len() - 1 {
                    heap.push(State { cost: cost + grid[y+1][x], x: x, y: y + 1, last_dir: Dir::Down, line_length: 1 });                    
                }
            },
            Dir::Left => {
                if line_length != max && x != 0 {
                    heap.push(State { cost: cost + grid[y][x-1], x: x - 1, y: y, last_dir: Dir::Left, line_length: line_length + 1 });                    
                }
                if line_length >= min && y != 0 {
                    heap.push(State { cost: cost + grid[y-1][x], x: x, y: y - 1, last_dir: Dir::Up, line_length: 1 });                    
                }
                if line_length >= min && y != grid.len() - 1 {
                    heap.push(State { cost: cost + grid[y+1][x], x: x, y: y + 1, last_dir: Dir::Down, line_length: 1 });                    
                }
            },
            Dir::Up => {
                if line_length != max && y != 0 {
                    heap.push(State { cost: cost + grid[y-1][x], x: x, y: y - 1, last_dir: Dir::Up, line_length: line_length + 1 });                    
                }
                if line_length >= min && x != 0 {
                    heap.push(State { cost: cost + grid[y][x-1], x: x-1, y: y, last_dir: Dir::Left, line_length: 1 });                    
                }
                if line_length >= min && x != grid[0].len() - 1 {
                    heap.push(State { cost: cost + grid[y][x+1], x: x+1, y: y, last_dir: Dir::Right, line_length: 1 });                    
                }
            },
            Dir::Down => {
                if line_length != max && y != grid.len() - 1 {
                    heap.push(State { cost: cost + grid[y+1][x], x: x, y: y + 1, last_dir: Dir::Down, line_length: line_length + 1 });                    
                }
                if line_length >= min && x != 0 {
                    heap.push(State { cost: cost + grid[y][x-1], x: x-1, y: y, last_dir: Dir::Left, line_length: 1 });                    
                }
                if line_length >= min && x != grid[0].len() - 1 {
                    heap.push(State { cost: cost + grid[y][x+1], x: x+1, y: y, last_dir: Dir::Right, line_length: 1 });                    
                }
            },
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<i64> {
    let grid = parse(input);
    solve(&grid, 0, 3)
}

pub fn part_two(input: &str) -> Option<i64> {
    let grid = parse(input);
    solve(&grid, 4, 10)
}

advent_of_code::main!(17);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 17));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 17));
        assert_eq!(result, Some(94));
    }
}
