use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Clone)]
#[derive(Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.split("\n").map(|s| s.chars().collect::<Vec<char>>()).collect()    
}

fn next_pos(pos: &(i32, i32), dir: Dir, grid: &Vec<Vec<char>>) -> Vec<((i32, i32),Dir)> {
    let mut ret_pos = Vec::new();
    match dir {
        Dir::Right => {
            if pos.0 < grid[0].len() as i32 - 1 {
                match grid[pos.1 as usize][(pos.0+1) as usize] {
                    '|' => {
                        ret_pos.push(((pos.0+1, pos.1),Dir::Up));
                        ret_pos.push(((pos.0+1, pos.1),Dir::Down));
                    }
                    '/' => {
                        ret_pos.push(((pos.0 + 1, pos.1), Dir::Up));
                    }
                    '\\' => {
                        ret_pos.push(((pos.0 + 1, pos.1), Dir::Down));
                    }
                    _ => {
                        ret_pos.push(((pos.0 + 1, pos.1), Dir::Right));
                    }
                }
            }
        },
        Dir::Left => {
            if pos.0 > 0 {
                match grid[pos.1 as usize][(pos.0-1) as usize] {
                    '|' => {
                        ret_pos.push(((pos.0-1, pos.1),Dir::Up));
                        ret_pos.push(((pos.0-1, pos.1),Dir::Down));
                    }
                    '/' => {
                        ret_pos.push(((pos.0 - 1, pos.1), Dir::Down));
                    }
                    '\\' => {
                        ret_pos.push(((pos.0 - 1, pos.1), Dir::Up));
                    }
                    _ => {
                        ret_pos.push(((pos.0 - 1, pos.1), Dir::Left));
                    }
                }
            }
        },
        Dir::Up => {
            if pos.1 > 0 {
                match grid[(pos.1-1) as usize][pos.0 as usize] {
                    '-' => {
                        ret_pos.push(((pos.0, pos.1 - 1),Dir::Left));
                        ret_pos.push(((pos.0, pos.1 - 1),Dir::Right));
                    }
                    '/' => {
                        ret_pos.push(((pos.0 , pos.1 - 1), Dir::Right));
                    }
                    '\\' => {
                        ret_pos.push(((pos.0, pos.1 - 1), Dir::Left));
                    }
                    _ => {
                        ret_pos.push(((pos.0, pos.1 - 1), Dir::Up));
                    }
                }
            }
        },
        Dir::Down => {
            if pos.1 < grid.len() as i32 - 1 {
                match grid[(pos.1+1) as usize][pos.0 as usize] {
                    '-' => {
                        ret_pos.push(((pos.0, pos.1 + 1),Dir::Left));
                        ret_pos.push(((pos.0, pos.1 + 1),Dir::Right));
                    }
                    '/' => {
                        ret_pos.push(((pos.0 , pos.1 + 1), Dir::Left));
                    }
                    '\\' => {
                        ret_pos.push(((pos.0, pos.1 + 1), Dir::Right));
                    }
                    _ => {
                        ret_pos.push(((pos.0, pos.1 + 1), Dir::Down));
                    }
                }
            }
        }
    }
    ret_pos
}

fn calc(pos: (i32, i32), dir: Dir, grid: &Vec<Vec<char>>) -> usize {
    let mut queue:VecDeque<((i32,i32),Dir)> = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((pos, dir));

    while let Some((pos, dir)) = queue.pop_front() {
        if seen.contains(&(pos,dir)) {
            continue;
        }
        if pos.0 >= 0 && pos.0 < grid[0].len() as i32 && pos.1 >= 0 && pos.1 < grid.len() as i32{
            seen.insert((pos,dir));
        }
        let new_pos = next_pos(&pos, dir, &grid);
        for p in new_pos {
            queue.push_back(p);
        }
    }
    let num_seen = seen.into_iter().map(|(p,_)| p).collect::<HashSet<(i32,i32)>>().len();
    num_seen
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    Some(calc((-1,0), Dir::Right, &grid))
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    let mut best = 0;
    for y in 0..grid.len() {
        best = best.max(calc((-1,y as i32), Dir::Right, &grid));
        best = best.max(calc((grid[y].len() as i32,y as i32), Dir::Left, &grid));
    }
    for x in 0..grid[0].len() {
        best = best.max(calc((x as i32, -1), Dir::Down, &grid));
        best = best.max(calc((x as i32, grid.len() as i32), Dir::Up, &grid));
    }
    Some(best)
}

advent_of_code::main!(16);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 16));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 16));
        assert_eq!(result, Some(51));
    }
}
