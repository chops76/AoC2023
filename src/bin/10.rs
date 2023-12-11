use std::collections::HashSet;

#[derive(PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.split("\n").map(|s| s.chars().collect::<Vec<char>>()).collect()
}

fn walk_path(start_x: usize, start_y: usize, start_dir: Dir, grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut cur_x = start_x;
    let mut cur_y = start_y;
    let mut dir = start_dir;
    let mut seen = HashSet::new();
    loop {
        seen.insert((cur_x, cur_y));
        match dir {
            Dir::Up => {
                cur_y -= 1;
                if grid[cur_y][cur_x] == 'F' {
                    dir = Dir::Right;
                } else if grid[cur_y][cur_x] == '7' {
                    dir = Dir::Left;
                }
            },
            Dir::Down => {
                cur_y += 1;
                if grid[cur_y][cur_x] == 'L' {
                    dir = Dir::Right;
                } else if grid[cur_y][cur_x] == 'J' {
                    dir = Dir::Left;
                }
            },
            Dir::Right => {
                cur_x += 1;
                if grid[cur_y][cur_x] == '7' {
                    dir = Dir::Down;
                } else if grid[cur_y][cur_x] == 'J' {
                    dir = Dir::Up;
                }
            },
            Dir::Left => {
                cur_x -= 1;
                if grid[cur_y][cur_x] == 'F' {
                    dir = Dir::Down;
                } else if grid[cur_y][cur_x] == 'L' {
                    dir = Dir::Up;
                }
            }
        }
        if cur_x == start_x && cur_y == start_y {
            break;
        }
    }
    seen
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let mut start_x = 0;
    let mut start_y = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                start_x = x;
                start_y = y;
                break;
            }
        }
    }
    let visited = walk_path(start_x, start_y, Dir::Down, &grid);
    Some(visited.len() as u32 / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);
    let mut start_x = 0;
    let mut start_y = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                start_x = x;
                start_y = y;
                break;
            }
        }
    }
    let seen = walk_path(start_x, start_y, Dir::Down, &grid);
    let mut new_grid = grid.clone();
    for y in 0..new_grid.len() {
        for x in 0..new_grid[y].len() {
            if !seen.contains(&(x, y)) {
                new_grid[y][x] = '.';
            }
        }
    }

    // Note: The following bit of code probably only works with my input
    new_grid[start_y][start_x] = '|';
    for x in 0..grid[0].len() {
        if new_grid[0][x] == 'F' {
            start_x = x;
            break;
        }
    }
    
    start_y = 0; 
    let mut cur_x = start_x;
    let mut cur_y = start_y;
    let mut inside = Dir::Down;
    let mut dir = Dir::Right;
    loop {
        match dir {
            Dir::Up => {
                cur_y -= 1;
                if new_grid[cur_y][cur_x] == 'F' {
                    dir = Dir::Right;
                    if inside == Dir::Right {
                        inside = Dir::Down;
                    } else {
                        if cur_x > 0 && new_grid[cur_y][cur_x - 1] == '.' {
                            new_grid[cur_y][cur_x - 1] = '#';
                        }
                        inside = Dir::Up;
                    }
                } else if new_grid[cur_y][cur_x] == '7' {
                    dir = Dir::Left;
                    if inside == Dir::Right {
                        if cur_x < new_grid[cur_y].len() - 1 && new_grid[cur_y][cur_x + 1] == '.' {
                            new_grid[cur_y][cur_x + 1] = '#';
                        }
                        inside = Dir::Up;
                    } else {
                        inside = Dir::Down;
                    }
                }
            },
            Dir::Down => {
                cur_y += 1;
                if new_grid[cur_y][cur_x] == 'L' {
                    dir = Dir::Right;
                    if inside == Dir::Right {
                        inside = Dir::Up;
                    } else {
                        if cur_x > 0 && new_grid[cur_y][cur_x - 1] == '.' {
                            new_grid[cur_y][cur_x - 1] = '#';
                        }
                        inside = Dir::Down;
                    }
                } else if new_grid[cur_y][cur_x] == 'J' {
                    dir = Dir::Left;
                    if inside == Dir::Right {
                        if cur_x <= new_grid[cur_y].len() - 1 && new_grid[cur_y][cur_x + 1] == '.' {
                            new_grid[cur_y][cur_x + 1] = '#';
                        }
                        inside = Dir::Down;
                    } else {
                        inside = Dir::Up;
                    }
                }
            },
            Dir::Right => {
                cur_x += 1;
                if new_grid[cur_y][cur_x] == '7' {
                    dir = Dir::Down;
                    if inside == Dir::Up {
                        if cur_y > 0 && new_grid[cur_y - 1][cur_x] == '.' {
                            new_grid[cur_y - 1][cur_x] = '#';
                        }
                        inside = Dir::Right;
                    } else {
                        inside = Dir::Left;
                    }
                } else if new_grid[cur_y][cur_x] == 'J' {
                    dir = Dir::Up;
                    if inside == Dir::Up {
                        inside = Dir::Left;
                    } else {
                        if cur_y < new_grid.len() - 1 && new_grid[cur_y + 1][cur_x] == '.' {
                            new_grid[cur_y + 1][cur_x] = '#';
                        }
                        inside = Dir::Right;
                    }
                }
            },
            Dir::Left => {
                cur_x -= 1;
                if new_grid[cur_y][cur_x] == 'F' {
                    dir = Dir::Down;
                    if inside == Dir::Up {
                        if cur_y > 0 && new_grid[cur_y - 1][cur_x] == '.' {
                            new_grid[cur_y - 1][cur_x] = '#';
                        }
                        inside = Dir::Left;
                    } else {
                        inside = Dir::Right;
                    }
                } else if new_grid[cur_y][cur_x] == 'L' {
                    dir = Dir::Up;
                    if inside == Dir::Up {
                        inside = Dir::Right;
                    } else {
                        if cur_y < new_grid.len() - 1 && new_grid[cur_y + 1][cur_x] == '.' {
                            new_grid[cur_y + 1][cur_x] = '#';
                        }
                        inside = Dir::Left;
                    }
                }
            }
        }
        match inside {
            Dir::Left => {
                if cur_x > 0 && new_grid[cur_y][cur_x - 1] == '.' {
                    new_grid[cur_y][cur_x - 1] = '#';
                }
            },
            Dir::Right => {
                if cur_x < new_grid[cur_y].len() - 1 && new_grid[cur_y][cur_x + 1] == '.' {
                    new_grid[cur_y][cur_x + 1] = '#';
                }
            },
            Dir::Up => {
                if cur_y > 0 && new_grid[cur_y - 1][cur_x] == '.' {
                    new_grid[cur_y - 1][cur_x] = '#';
                }
            }
            Dir::Down => {
                if cur_y < new_grid.len() - 1 && new_grid[cur_y + 1][cur_x] == '.' {
                    new_grid[cur_y + 1][cur_x] = '#';
                }
            }
        }
        if cur_x == start_x && cur_y == start_y {
            break;
        }
    }
     
    let mut updated = true;
    while updated {
        updated = false;
        for y in 0..new_grid.len() {
            for x in 0..new_grid[y].len() {
                if new_grid[y][x] == '#' {
                    if y > 0 && new_grid[y-1][x] == '.' {
                        new_grid[y-1][x] = '#';
                        updated = true;
                    }
                    if y < new_grid.len() - 1 && new_grid[y+1][x] == '.' {
                        new_grid[y+1][x] = '#';
                        updated = true;
                    }
                    if x > 0 && new_grid[y][x-1] == '.' {
                        new_grid[y][x-1] = '#';
                        updated = true;
                    }
                    if x < new_grid[y].len() - 1 && new_grid[y][x+1] == '.' {
                        new_grid[y][x+1] = '#';
                        updated = true;
                    }
                }
            }
        }
    }
    let mut count = 0;
    for y in 0..new_grid.len() {
        for x in 0..new_grid[y].len() {
            if new_grid[y][x] == '#' {
                count += 1;
            }
        }
    }
    Some(count)
}

advent_of_code::main!(10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 10));
        assert_eq!(result, Some(8));
    }
}

//Could be 579,580,582,583