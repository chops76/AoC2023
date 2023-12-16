fn parse(input: &str) -> Vec<String> {
    input.split(",").map(|s| s.to_string()).collect()
}

fn hash(s: &str) -> u64 {
    s.chars().fold(0, |acc, c| ((acc + c as u64) * 17) % 256)
}

pub fn part_one(input: &str) -> Option<u64> {
    let commands = parse(input);
    Some(commands.iter().map(|s| hash(s)).sum())
}

fn insert(label: &str, val: u64, hm: &mut Vec<Vec<(String, u64)>>) {
    let hash = hash(label) as usize;
    let mut replaced = false;
    for i in 0..hm[hash].len() {
        if hm[hash][i].0 == label {
            hm[hash][i].1 = val;
            replaced = true;
            break;
        }
    }
    if !replaced {
        hm[hash].push((label.to_string(), val));
    }
}

fn remove(label: &str, hm: &mut Vec<Vec<(String, u64)>>) {
    let hash = hash(label) as usize;
    for i in 0..hm[hash].len() {
        if hm[hash][i].0 == label {
            hm[hash].remove(i);
            break;
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let commands = parse(input);
    let mut hm = vec![Vec::new(); 256];
    for cmd in commands {
        if cmd.contains('=') {
            let spl = cmd.split("=").collect::<Vec<&str>>();
            insert(spl[0], spl[1].parse().unwrap(), &mut hm);
        } else {
            let spl = cmd.split("-").collect::<Vec<&str>>();
            remove(spl[0], &mut hm);
        }
    }

    let mut sum = 0;
    for box_num in 0..hm.len() {
        for lens in 0..hm[box_num].len() {
            sum += (box_num as u64 + 1) * (lens as u64 + 1) * hm[box_num][lens].1;
        }
    }
    Some(sum)
}

advent_of_code::main!(15);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 15));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 15));
        assert_eq!(result, Some(145));
    }
}
