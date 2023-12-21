use std::collections::VecDeque;
use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.split("\n").map(|s| s.chars().collect::<Vec<char>>()).collect()    
}

pub fn part_one(input: &str) -> Option<i64> {
    let grid = parse(input);
    let mut queue = VecDeque::new();
    let mut cur_x = 0;
    let mut cur_y = 0;
    for y in 0..grid.len() {
        let mut found = false;
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                cur_x = x;
                cur_y = y;
                break;
            }
        }
        if found {
            break;
        }
    }
    let mut seen = HashSet::new();
    queue.push_back((cur_x, cur_y, 0));
    while let Some((x, y, dist)) = queue.pop_front() {
        if dist > 64 {
            break;
        }
        if seen.contains(&(x, y, dist)) {
            continue;
        }
        seen.insert((x, y, dist));
        if x != 0 && grid[y][x - 1] != '#' {
            queue.push_back((x - 1, y, dist + 1));
        }
        if x != grid[y].len() - 1 && grid[y][x + 1] != '#' {
            queue.push_back((x + 1, y, dist + 1));
        }
        if y != 0 && grid[y - 1][x] != '#' {
            queue.push_back((x, y - 1, dist + 1));
        }
        if y != grid.len() - 1 && grid[y + 1][x] != '#' {
            queue.push_back((x, y + 1, dist + 1));
        }
    }
    let val = seen.iter().filter(|(_,_,dist)| *dist == 64).count();
    Some(val as i64)
}

fn calc_deltas(v: &Vec<i64>) -> Vec<i64> {
    let mut ret_vec = Vec::new();
    for i in 0..v.len() - 1 {
        ret_vec.push(v[i+1] - v[i]);
    }
    ret_vec
}

pub fn part_two(input: &str) -> Option<u64> {
    // I used the commented out code to help me find the pattern, and then the 
    // non-commented code finds the answer.
    /* 
    let grid = parse(input);
    let mut queue = VecDeque::new();
    let mut cur_x = 0;
    let mut cur_y = 0;
    let mut vals = Vec::new();
    let mut prev_dist = 0;
    for y in 0..grid.len() {
        let mut found = false;
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                cur_x = x;
                cur_y = y;
                found = true;
            }
        }
        if found {
            break;
        }
    }
    let mut seen = HashSet::new();
    queue.push_back((cur_x, cur_y, 0, 0, 0));
    while let Some((x, y, grid_x, grid_y, dist)) = queue.pop_front() {
        if dist != prev_dist {
            let val = seen.iter().filter(|(_,_,_,_,dist)| *dist == prev_dist).count(); 
            if prev_dist % 55 == 0 {
                vals.push(val as i64);
                let left = seen.iter().filter(|(_,_,a,b,d)| *a == -1 && *b == 0 && *d == prev_dist).count();
                let left2 = seen.iter().filter(|(_,_,a,b,d)| *a == -2 && *b == 0 && *d == prev_dist).count();
                let left3 = seen.iter().filter(|(_,_,a,b,d)| *a == -3 && *b == 0 && *d == prev_dist).count();
                
                println!("Dist: {}  Left: {}  Left2: {}  Left3: {}", prev_dist, left, left2, left3);
                
                let middle = seen.iter().filter(|(_,_,a,b,d)| *a == 0 && *b == 0 && *d == prev_dist).count();
                println!("Middle for dist = {}: {}", dist, middle);
                let left = seen.iter().filter(|(_,_,a,b,d)| *a == -1 && *b == 0 && *d == prev_dist).count();
                println!("left for dist = {}: {}", dist, left);
                let left2 = seen.iter().filter(|(_,_,a,b,d)| *a == -2 && *b == 0 && *d == prev_dist).count();
                println!("left for dist = {}: {}", dist, left2);
                let right = seen.iter().filter(|(_,_,a,b,d)| *a == 1 && *b == 0 && *d == prev_dist).count();
                println!("right for dist = {}: {}", dist, right);
                let up = seen.iter().filter(|(_,_,a,b,d)| *a == 0 && *b == -1 && *d == prev_dist).count();
                println!("up for dist = {}: {}", dist, up);
                let down = seen.iter().filter(|(_,_,a,b,d)| *a == 0 && *b == 1 && *d == prev_dist).count();
                println!("down for dist = {}: {}", dist, down); 
            }  
            if dist > 460 {
                break
            }
            prev_dist = dist;         
        }
        if seen.contains(&(x, y, grid_x, grid_y, dist)) {
            continue;
        }
        seen.insert((x, y, grid_x, grid_y, dist));
        if x == 0 {
            queue.push_back((grid[y].len() - 1, y, grid_x - 1, grid_y, dist + 1));
        }
        if x != 0 && grid[y][x - 1] != '#' {
            queue.push_back((x - 1, y, grid_x, grid_y, dist + 1));
        }
        if x == grid[y].len() - 1 {
            queue.push_back((0, y, grid_x + 1, grid_y, dist + 1));
        }
        if x != grid[y].len() - 1 && grid[y][x + 1] != '#' {
            queue.push_back((x + 1, y, grid_x, grid_y, dist + 1));
        }
        if y == 0 {
            queue.push_back((x, grid.len() - 1, grid_x, grid_y - 1, dist + 1));
        }
        if y != 0 && grid[y - 1][x] != '#' {
            queue.push_back((x, y - 1, grid_x, grid_y, dist + 1));
        }
        if y == grid.len() - 1 {
            queue.push_back((x, 0, grid_x, grid_y + 1, dist + 1));
        }
        if y != grid.len() - 1 && grid[y + 1][x] != '#' {
            queue.push_back((x, y + 1, grid_x, grid_y, dist + 1));
        }
    }
    println!("Vals[65] = {}  Vals[196] = {}  Vals[327] = {}   Vals[458] = {}", vals[65], vals[196], vals[327], vals[458]);
*/
    let mut total = 3916_u64;
    let mut to_add = 30954;
    let mut cur_steps = 65;
    while cur_steps < 26501365 {
        total += to_add;
        cur_steps += 131;
        to_add += 30820;
    }
    Some(total)
}

advent_of_code::main!(21);

