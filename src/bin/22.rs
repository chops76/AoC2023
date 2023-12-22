use rayon::prelude::*;

use std::collections::HashSet;

fn parse(input: &str) -> Vec<((i32,i32,i32),(i32,i32,i32))> {
    let mut ret_vec = Vec::new();
    for l in input.split("\n") {
        let spl = l.split("~").collect::<Vec<&str>>();
        let first = spl[0].split(",").map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
        let second = spl[1].split(",").map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
        ret_vec.push(((first[0], first[1], first[2]),(second[0], second[1], second[2])));
    }

    ret_vec
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut blocks = parse(input);
    blocks.sort_by_key(|((_,_,a),(_,_,b))| *a.min(b));
    let mut trench = vec![vec![vec![-1;400];400];400];
    let mut rested_on: HashSet<i32> = HashSet::new();
    for x in 0..400 {
        for y in 0..400 {
            trench[x][y][0] = i32::MAX;
        }
    }
    for i in 0..blocks.len() {
        let min_x = blocks[i].0.0.min(blocks[i].1.0);
        let max_x = blocks[i].0.0.max(blocks[i].1.0);
        let min_y = blocks[i].0.1.min(blocks[i].1.1);
        let max_y = blocks[i].0.1.max(blocks[i].1.1);
        let start_z = blocks[i].0.2.min(blocks[i].1.2);  
        let mut z = start_z;
        loop {
            let mut resting = false;
            let mut hs = HashSet::new();
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    let below = trench[x as usize][y as usize][(z - 1) as usize];
                    if below != -1 {
                        if below != i32::MAX {
                            hs.insert(below);
                        }
                        resting = true;
                    }
                }
            }
            if hs.len() == 1 {
                rested_on.extend(&hs);
            }
            if resting {
                break;
            }
            z -= 1;
        }    
        let dropped = start_z - z;
        blocks[i].0.2 -= dropped;
        blocks[i].1.2 -= dropped;
        let min_z = blocks[i].0.2.min(blocks[i].1.2);
        let max_z = blocks[i].0.2.max(blocks[i].1.2);
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    trench[x as usize][y as usize][z as usize] = i as i32;
                }
            }
        }
    }
    Some(blocks.len() - rested_on.len())
}

fn drop_bricks(blocks: &Vec<((i32, i32, i32), (i32, i32, i32))>) -> (usize, Vec<((i32, i32, i32), (i32, i32, i32))>) {
    let mut blocks = blocks.clone();
    blocks.sort_by_key(|((_,_,a),(_,_,b))| *a.min(b));
    let mut trench = vec![vec![vec![-1;400];400];400];
    let mut num_dropped = 0;
    for x in 0..400 {
        for y in 0..400 {
            trench[x][y][0] = i32::MAX;
        }
    }
    for i in 0..blocks.len() {
        let min_x = blocks[i].0.0.min(blocks[i].1.0);
        let max_x = blocks[i].0.0.max(blocks[i].1.0);
        let min_y = blocks[i].0.1.min(blocks[i].1.1);
        let max_y = blocks[i].0.1.max(blocks[i].1.1);
        let start_z = blocks[i].0.2.min(blocks[i].1.2);  
        let mut z = start_z;
        loop {
            let mut resting = false;
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    let below = trench[x as usize][y as usize][(z - 1) as usize];
                    if below != -1 {
                        resting = true;
                    }
                }
            }
            if resting {
                break;
            }
            z -= 1;
        }    
        let dropped = start_z - z;
        if dropped > 0 {
            num_dropped += 1;
        }
        blocks[i].0.2 -= dropped;
        blocks[i].1.2 -= dropped;
        let min_z = blocks[i].0.2.min(blocks[i].1.2);
        let max_z = blocks[i].0.2.max(blocks[i].1.2);
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    trench[x as usize][y as usize][z as usize] = i as i32;
                }
            }
        }
    }
    (num_dropped, blocks)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut blocks = parse(input);
    (_, blocks) = drop_bricks(&blocks);
    let mut queue = Vec::new();
    for i in 0..blocks.len() {
        let mut test_blocks = blocks.clone();
        test_blocks.remove(i);
        queue.push(test_blocks);
    }
    let dropped = queue.par_iter().map(|blocks| drop_bricks(&blocks).0).sum();
    Some(dropped)
}

advent_of_code::main!(22);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 22));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 22));
        assert_eq!(result, Some(7));
    }
}
