use std::collections::HashMap;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.split_whitespace().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
}

fn get_num_positions(grid: &Vec<Vec<char>>) -> Vec<(usize, usize, usize)> {
    let mut ret_vec = Vec::new();
    for y in 0..grid.len() {
        let mut in_num = false;
        let mut x_start = 0;
        for x in 0..grid[y].len() {
            if !in_num && grid[y][x].is_ascii_digit() {
                in_num = true;
                x_start = x;
            }
            if in_num && !grid[y][x].is_ascii_digit() {
                in_num = false;
                ret_vec.push((x_start, y, x - x_start));
            }
            if in_num && grid[y][x].is_ascii_digit() && x == grid[y].len() - 1 {
                in_num = false;
                ret_vec.push((x_start, y, x - x_start + 1));
            }
        }
    }
    ret_vec
}

fn symbol_adjacent(x: usize, y: usize, len: usize, grid: &Vec<Vec<char>>) -> bool {
    let mut to_check = Vec::new();
    for i in ((x as i32) - 1)..=((x + len) as i32) {
        to_check.push((i, y as i32 - 1));
        to_check.push((i, y as i32 + 1));
    }
    to_check.push((x as i32 - 1, y as i32));
    to_check.push(((x + len) as i32, y as i32));
    for pos in to_check {
        if pos.0 < 0 || pos.1 < 0 {
            continue;
        }
        let x = pos.0 as usize;
        let y = pos.1 as usize;
        if y >= grid.len() || x >= grid[0].len() {
            continue;
        }
        if !grid[y][x].is_ascii_digit() && grid[y][x] != '.' {
            return true;
        }
    }
    false
}

fn get_value(x: usize, y: usize, len: usize, grid: &Vec<Vec<char>>) -> u32 {
    let mut val = 0;
    for i in 0..len {
        val *= 10;
        val += grid[y][x + i].to_digit(10).unwrap();
    }
    val
}

fn get_adjacent_symbols(x: usize, y: usize, len: usize, grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut ret_vec = Vec::new();
    let mut to_check = Vec::new();
    for i in ((x as i32) - 1)..=((x + len) as i32) {
        to_check.push((i, y as i32 - 1));
        to_check.push((i, y as i32 + 1));
    }
    to_check.push((x as i32 - 1, y as i32));
    to_check.push(((x + len) as i32, y as i32));
    for pos in to_check {
        if pos.0 < 0 || pos.1 < 0 {
            continue;
        }
        let x = pos.0 as usize;
        let y = pos.1 as usize;
        if y >= grid.len() || x >= grid[0].len() {
            continue;
        }
        if grid[y][x] == '*' {
            ret_vec.push((x, y));
        }
    }
    ret_vec
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let nums = get_num_positions(&grid);
    let mut sum = 0;
    for num in &nums {
        if symbol_adjacent(num.0, num.1, num.2, &grid) {
            sum += get_value(num.0, num.1, num.2, &grid);
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hm = HashMap::new();
    let grid = parse(input);
    let nums = get_num_positions(&grid);
    for num in &nums {
        let symbols = get_adjacent_symbols(num.0, num.1, num.2, &grid);
        for s in symbols {
            if !hm.contains_key(&s) {
                hm.insert(s, Vec::new());
            }
            hm.get_mut(&s).unwrap().push(num.clone());
        }
    }
    let valid = hm.values().filter(|v| v.len() == 2).collect::<Vec<&Vec<(usize, usize, usize)>>>();
    let mut sum = 0;
    for v in valid {
        let val1 = get_value(v[0].0, v[0].1, v[0].2, &grid);
        let val2 = get_value(v[1].0, v[1].1, v[1].2, &grid);
        sum += val1 * val2;
    }
    Some(sum)
}

advent_of_code::main!(3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, Some(467835));
    }
}
