use num::integer::lcm;
use std::collections::HashMap;

fn parse(input: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let spl = input.split("\n\n").collect::<Vec<&str>>();
    let dirs = spl[0].chars().collect::<Vec<char>>();
    let mut hm = HashMap::new();
    for l in spl[1].lines() {
        let node_spl = l.split(" = (").collect::<Vec<&str>>();
        hm.insert(node_spl[0].to_string(), (node_spl[1][..3].to_string(), node_spl[1][5..8].to_string()));
    }
    (dirs, hm)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (dirs, nodes) = parse(input);
    Some(find_cycle("AAA", &dirs, &nodes))
}

fn find_cycle(node: &str, dirs: &Vec<char>, nodes: &HashMap<String, (String, String)>) -> usize {
    let mut cur = node.clone();
    let mut dir_count = 0;
    loop {
        if dirs[dir_count % dirs.len()] == 'R' {
            cur = &nodes[cur].1;
        } else {
            cur = &nodes[cur].0;
        }
        dir_count += 1;
        if cur.chars().nth(2).unwrap() == 'Z' {
            return dir_count;
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (dirs, nodes) = parse(input);
    let mut cycles = Vec::new();
    for n in nodes.keys() {
        if n.chars().nth(2).unwrap() == 'A' {
            cycles.push(find_cycle(n, &dirs, &nodes) as u64);
        }
    } 
    let mut answer = cycles[0];
    for i in 1..cycles.len() {
        answer = lcm(answer, cycles[i]);
    }
    Some(answer)
}

advent_of_code::main!(8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 8));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 8));
        assert_eq!(result, Some(6));
    }
}
