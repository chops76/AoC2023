use std::collections::HashSet;
use std::collections::HashMap;
use rand::seq::SliceRandom;

fn parse(input: &str) -> (HashSet<String>, HashSet<(String, String)> ){
    let mut vert = HashSet::new();
    let mut edge = HashSet::new();
    for s in input.split("\n") {
        let spl = s.split(": ").collect::<Vec<&str>>();
        vert.insert(spl[0].to_string());
        for conn in spl[1].split_whitespace() {
            vert.insert(conn.to_string());
            if !edge.contains(&(spl[0].to_string(), conn.to_string())) &&
               !edge.contains(&(conn.to_string(), spl[0].to_string())) {
                edge.insert((conn.to_string(), spl[0].to_string()));
               }
        }
    }
    (vert, edge)
}

fn karger(verticies: &Vec<String>, edges: &Vec<(String, String)>) -> (usize, usize)
{
    let mut hs = HashMap::new();
    for v in verticies {
        hs.insert(v.to_string(), (v.to_string(), 0));
    }

    let mut v_count = verticies.len();
    while v_count > 2
    {
        let edge = edges.choose(&mut rand::thread_rng()).unwrap();

        let subset1 = find(&mut hs, &edge.0);
        let subset2 = find(&mut hs, &edge.1);

        if subset1 == subset2 {
            continue;
        }

        union(&mut hs, &subset1, &subset2);
        v_count -= 1;
    }

    let mut hm = HashMap::new();
    let mut cut_count = 0;
    for v in verticies {
        let ss = find(&mut hs, v);
        if !hm.contains_key(&ss) {
            hm.insert(ss, 1);
        } else {
            *hm.get_mut(&ss).unwrap() += 1;
        }
    }
    
    for edge in edges {
        let subset1 = find(&mut hs, &edge.0);
        let subset2 = find(&mut hs, &edge.1);
        if subset1 != subset2 {
            cut_count += 1;
        }
    }

    (cut_count, hm.values().product())
}

fn find(subsets: &mut HashMap<String, (String, i32)>, i: &str) -> String
{
    let par = subsets[i].0.clone();
    if par != i {
        let new_parent = find(subsets, &par);
        subsets.get_mut(i).unwrap().0 = new_parent;
    }
    
    return subsets[i].0.clone();
}

fn union(subsets: &mut HashMap<String, (String, i32)>, x: &str, y: &str)
{
    let xroot = find(subsets, x);
    let yroot = find(subsets, y);
 
    if subsets[&xroot].1 < subsets[&yroot].1 {
        subsets.get_mut(&xroot).unwrap().0 = yroot;
    } else if subsets[&xroot].1 > subsets[&yroot].1 {
        subsets.get_mut(&yroot).unwrap().0 = xroot;     
    } else {
        subsets.get_mut(&yroot).unwrap().0 = xroot.clone();  
        subsets.get_mut(&xroot).unwrap().1 += 1;  
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (verticies, edges) = parse(input);
    let v_vec = verticies.into_iter().collect::<Vec<String>>();
    let e_vec = edges.into_iter().collect::<Vec<(String, String)>>();
    let (mut cuts, mut product) = karger(&v_vec, &e_vec);
    while cuts > 3 {
        (cuts, product) = karger(&v_vec, &e_vec);
    }
    Some(product)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(25);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 25));
        assert_eq!(result, Some(54));
    }
}
