use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.split("\n").map(|s| s.chars().collect::<Vec<char>>()).collect()    
}

fn roll(grid: &mut Vec<Vec<char>>) {
    for i in 1..grid.len() {
        for j in (1..=i).rev() {
            for col in 0..grid[j].len() {
                if grid[j][col] == 'O' && grid[j-1][col] == '.' {
                    grid[j][col] = '.';
                    grid[j-1][col] = 'O';
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = parse(input);
    roll(&mut grid);
    let mut sum = 0;
    for i in 0..grid.len() {
        sum += grid[i].iter().filter(|c| **c == 'O').count() * (grid.len() - i);
    }
    
    Some(sum)
}

fn rotate(grid: &mut Vec<Vec<char>>) {
    let mut new_grid = Vec::new();
    for x in 0..grid[0].len() {
        let mut new_row = Vec::new();
        for y in (0..grid.len()).rev() {
            new_row.push(grid[y][x]);
        }
        new_grid.push(new_row);
    }
    *grid = new_grid;
}

fn spin(grid: &mut Vec<Vec<char>>) {
    for _ in 0..4 {
        roll(grid);
        rotate(grid);
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse(input);
    let mut hs = HashSet::new();
    let mut offset = 0;
    for i in 0..10000 {
        if hs.contains(&grid) {
            offset = i;
            break;
        }
        hs.insert(grid.clone());
        spin(&mut grid);
    }
    let mut cycle = 0;
    let mut hs = HashSet::new();
    for i in 0..10000 {
        if hs.contains(&grid) {
            cycle = i;
            break;
        }
        hs.insert(grid.clone());
        spin(&mut grid);
    }
    let to_calc = (1000000000_u64 - offset) % cycle;
    for _ in 0..to_calc {
        spin(&mut grid);
    }
    let mut sum = 0;
    for i in 0..grid.len() {
        sum += grid[i].iter().filter(|c| **c == 'O').count() * (grid.len() - i);
    }
    Some(sum)
}

advent_of_code::main!(14);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 14));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 14));
        assert_eq!(result, Some(64));
    }
}
