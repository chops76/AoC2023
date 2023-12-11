fn parse(input: &str) -> Vec<Vec<char>> {
    input.split("\n").map(|s| s.chars().collect::<Vec<char>>()).collect()    
}

fn calc(size: u64, grid: &Vec<Vec<char>>) -> u64 {
    let mut row_size = vec![1; grid.len()];
    let mut col_size = vec![1; grid[0].len()];
    for row in 0..grid.len() {
        if grid[row].iter().all(|c| *c == '.') {
            row_size[row] = size;
        }
    }
    for col in 0..grid[0].len() {
        let mut clear = true;
        for row in 0..grid.len() {
            if grid[row][col] != '.' {
                clear = false;
                break;
            }
        }
        if clear {
            col_size[col] = size;
        }
    }
    let mut galaxies = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != '.' {
                galaxies.push((x, y));
            }
        }
    }
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let min_x = galaxies[i].0.min(galaxies[j].0);
            let max_x = galaxies[i].0.max(galaxies[j].0);
            let min_y = galaxies[i].1.min(galaxies[j].1);
            let max_y = galaxies[i].1.max(galaxies[j].1);
            for x in min_x + 1..=max_x {
                sum += col_size[x];
            }
            for y in min_y + 1..=max_y {
                sum += row_size[y];
            }
        }
    }

    sum
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse(input);

    Some(calc(2, &grid))
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse(input);

    Some(calc(1000000, &grid))
}

advent_of_code::main!(11);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 11));
        assert_eq!(result, Some(374));
    }
}
