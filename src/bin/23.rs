use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.split("\n").map(|s| s.chars().collect::<Vec<char>>()).collect()    
}

pub fn part_one(input: &str) -> Option<u32> { 
    let grid = parse(input);
    let start_y = 0;
    let mut start_x = 0;
    for x in 0..grid[start_y].len() {
        if grid[start_y][x] == '.' {
            start_x = x;
            break;
        }
    }
    let end_y = grid.len() - 1;
    let mut end_x = 0;
    for x in 0..grid[end_y].len() {
        if grid[end_y][x] == '.' {
            end_x = x;
            break;
        }
    }
    let mut best = HashMap::new();
    let mut queue = VecDeque::new();
    let mut max_dist = 0;
    queue.push_back((start_x, start_y, 0, HashSet::new()));
    while let Some((x, y, dist, seen)) = queue.pop_front() {
        if seen.contains(&(x, y)) {
            continue;
        }
        if best.contains_key(&(x, y)) {
            if best[&(x,y)] >= dist {
                continue;
            } else {
                *best.get_mut(&(x,y)).unwrap() = dist;
            }
        } else {
            best.insert((x, y), dist);
        }
        let mut new_seen = seen.clone();
        new_seen.insert((x, y));
        if x == end_x && y == end_y {
            max_dist = max_dist.max(dist);
        }
        if grid[y][x] == '.' {
            if y > 0 && grid[y-1][x] != '#' {
                queue.push_back((x, y-1, dist+1, new_seen.clone()));
            }
            if y < grid.len() - 1 && grid[y+1][x] != '#' {
                queue.push_back((x, y+1, dist+1, new_seen.clone()));
            }
            if x > 0 && grid[y][x-1] != '#' {
                queue.push_back((x-1, y, dist+1, new_seen.clone()));
            }
            if x < grid[y].len() - 1 && grid[y][x+1] != '#' {
                queue.push_back((x+1, y, dist+1, new_seen.clone()));
            }
        }
        if grid[y][x] == '^' && y > 0 && grid[y-1][x] != '#' {
            queue.push_back((x, y-1, dist+1, new_seen.clone()));
        }
        if grid[y][x] == 'v' && y < grid.len() - 1 && grid[y+1][x] != '#' {
            queue.push_back((x, y+1, dist+1, new_seen.clone()));
        }
        if grid[y][x] == '<' && x > 0 && grid[y][x-1] != '#' {
            queue.push_back((x-1, y, dist+1, new_seen.clone()));
        }
        if grid[y][x] == '>' && x < grid[y].len() - 1 && grid[y][x+1] != '#' {
            queue.push_back((x+1, y, dist+1, new_seen.clone()));
        }   
    }
    Some(max_dist) 
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);
    let start_y = 0;
    let mut start_x = 0;
    for x in 0..grid[start_y].len() {
        if grid[start_y][x] == '.' {
            start_x = x;
            break;
        }
    }
    let end_y = grid.len() - 1;
    let mut end_x = 0;
    for x in 0..grid[end_y].len() {
        if grid[end_y][x] == '.' {
            end_x = x;
            break;
        }
    }

    let mut nodes = HashSet::new();
    nodes.insert((start_x, start_y));
    nodes.insert((end_x, end_y));

    for y in 1..grid.len() - 1 {
        for x in 1..grid.len() - 1 {
            if grid[y][x] != '#' {
                let mut num = 0;
                if grid[y-1][x] != '#' {
                    num += 1;
                }
                if grid[y+1][x] != '#' {
                    num += 1;
                }
                if grid[y][x-1] != '#' {
                    num += 1;
                }
                if grid[y][x+1] != '#' {
                    num += 1;
                }
                if num > 2 {
                    nodes.insert((x, y));
                }
            }
        }
    }
    let mut hm = HashMap::new();
    for (node_x, node_y) in &nodes {
        let mut connections = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((*node_x, *node_y, 0, HashSet::new()));
        while let Some((x, y, dist, seen)) = queue.pop_front() {
            if seen.contains(&(x, y)) {
                continue;
            }
            if !(x == *node_x && y == *node_y) && nodes.contains(&(x, y)) {
                connections.push((x, y, dist));
                continue;   
            }
            let mut new_seen = seen.clone();
            new_seen.insert((x, y));

            if y > 0 && grid[y-1][x] != '#' {
                queue.push_back((x, y-1, dist+1, new_seen.clone()));
            }
            if y < grid.len() - 1 && grid[y+1][x] != '#' {
                queue.push_back((x, y+1, dist+1, new_seen.clone()));
            }
            if x > 0 && grid[y][x-1] != '#' {
                queue.push_back((x-1, y, dist+1, new_seen.clone()));
            }
            if x < grid[y].len() - 1 && grid[y][x+1] != '#' {
                queue.push_back((x+1, y, dist+1, new_seen.clone()));
            }
        }     
        hm.insert((node_x, node_y), connections);
    }
    let mut queue = VecDeque::new();
    let mut max_dist = 0;
    queue.push_back((start_x, start_y, 0, HashSet::new()));
    while let Some((x, y, dist, seen)) = queue.pop_front() {
        if seen.contains(&(x, y)) {
            continue;
        }
        let mut new_seen = seen.clone();
        new_seen.insert((x, y));
        if x == end_x && y == end_y {
            max_dist = max_dist.max(dist);
        }
        for (adj_x, adj_y, adj_dist) in &hm[&(&x,&y)] {
            queue.push_back((*adj_x, *adj_y, dist+adj_dist, new_seen.clone()));
        }
    }
    Some(max_dist)
}

advent_of_code::main!(23);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 23));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 23));
        assert_eq!(result, Some(154));
    }
}
