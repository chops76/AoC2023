fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    input.split("\n\n").map(|g| g.split("\n").map(|s| s.chars().collect::<Vec<char>>()).collect()).collect()   
}

fn differences(s1: &Vec<char>, s2: &Vec<char>) -> usize {
    let mut count = 0;
    for i in 0..s1.len() {
        if s1[i] != s2[i] {
            count += 1;
        }
    }
    count
}

fn reflect(grid: &Vec<Vec<char>>, goal: usize) -> Option<usize> {
    for row in 0..grid.len() - 1 {
        let mut count = 0;
        let mut top = row;
        let mut bottom = row + 1;
        loop {
            count += differences(&grid[top], &grid[bottom]);
            if top == 0 || bottom == grid.len() - 1 || count > goal {
                break;
            }
            top -= 1;
            bottom += 1;
        }
        if count == goal {
            return Some(row + 1);
        }
    }
    None
}

fn flip(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut ret_vec = Vec::new();
    for col in 0..grid[0].len() {
        let mut tmp_vec = Vec::new();
        for row in 0..grid.len() {
            tmp_vec.push(grid[row][col]);
        }
        ret_vec.push(tmp_vec);
    }
    ret_vec
}

pub fn part_one(input: &str) -> Option<usize> {
    let grids = parse(input);
    let mut sum = 0;
    for grid in &grids {
        if let Some(val) = reflect(&grid, 0) {
            sum += 100 * val;
        }
        if let Some(val) = reflect(&flip(grid), 0) {
            sum += val;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grids = parse(input);
    let mut sum = 0;
    for grid in &grids {
        if let Some(val) = reflect(&grid, 1) {
            sum += 100 * val;
        }
        if let Some(val) = reflect(&flip(grid), 1) {
            sum += val;
        }
    }
    Some(sum)
}

advent_of_code::main!(13);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 13));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 13));
        assert_eq!(result, Some(400));
    }
}
