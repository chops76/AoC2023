use std::collections::HashMap;

fn parse(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    let mut ret_vec = Vec::new();
    for s in input.split("\n") {
        let spl = s.split_whitespace().collect::<Vec<&str>>();
        ret_vec.push((spl[0].chars().collect(), spl[1].split(",").map(|v| v.parse().unwrap()).collect()));
    }
    ret_vec
}

fn recur(record: &Vec<char>, pos: usize, parts: &Vec<usize>, used: usize, cache: &mut HashMap<(usize, usize), u64>) -> u64 {
    if cache.contains_key(&(pos, used)) {
        return cache[&(pos, used)];
    }
    if used == parts.len() {
        let mut valid = true;
        for i in pos..record.len() {
            if record[i] == '#' {
                valid = false;
                break;
            }
        }
        let num = if valid { 1 } else { 0 };
        cache.insert((pos, used), num);
        return num;
    }
    let to_fit: usize = parts[used..].iter().sum::<usize>() + (parts.len() - used - 1);
    if to_fit > (record.len() - pos) {
        cache.insert((pos, used), 0);
        return 0;
    }
    let mut sum = 0;
    if record[pos] != '#' {
        sum += recur(record, pos + 1, parts, used, cache);
    }
    let mut valid = true;
    for i in 0..parts[used] {
        if record[pos + i] == '.' {
            valid = false;
            break;
        }
    }
    let next_pos = pos + parts[used];
    if next_pos < record.len() && record[next_pos] == '#' {
        valid = false;
    }
    if valid {
        sum += recur(record, next_pos + 1, parts, used + 1, cache);
    }
    cache.insert((pos, used), sum);
    sum
}

pub fn part_one(input: &str) -> Option<u64> {
    let rows = parse(input);
    
    let mut sum = 0; 
    
    for row in &rows {
        let mut cache = HashMap::new();
        sum += recur(&row.0, 0, &row.1, 0, &mut cache);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> { 
    let rows = parse(input);
    let mut sum = 0;
    for row in &rows {
        let mut new_row = Vec::new();
        for i in 0..5 {
            if i != 0 {
                new_row.push(',');
            }
            for j in 0..row.0.len() {
                new_row.push(row.0[j]);
            }
        }
        let new_parts: Vec<_> = row.1.clone().into_iter().cycle().take(row.1.len() * 5).collect();
        let mut cache = HashMap::new();
        let val = recur(&new_row, 0, &new_parts, 0, &mut cache);
        sum += val;
    }
    Some(sum)
}

advent_of_code::main!(12);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 12));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 12));
        assert_eq!(result, Some(525152));
    }
}
