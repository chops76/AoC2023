fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    let spl = input.split("\n").collect::<Vec<&str>>();
    let time_spl = spl[0].split(":").collect::<Vec<&str>>();
    let times = time_spl[1].split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let dist_spl = spl[1].split(":").collect::<Vec<&str>>();
    let dists = dist_spl[1].split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    (times, dists)
}

fn parse2(input: &str) -> (i64, i64) {
    let spl = input.split("\n").collect::<Vec<&str>>();
    let time_spl = spl[0].split(":").collect::<Vec<&str>>();
    let time = time_spl[1].trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("").parse::<i64>().unwrap();
    
    let dist_spl = spl[1].split(":").collect::<Vec<&str>>();
    let dist = dist_spl[1].trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("").parse::<i64>().unwrap();
    (time, dist)
}

fn get_roots(time: i64, dist: i64) -> (i64, i64) {
    let time = time as f64;
    let dist = dist as f64;
    let cong = (time * time - 4.0 * dist).sqrt();
    let r1 = (time + cong) / 2.0;
    let r2 = (time - cong) / 2.0;
    let lower = r1.min(r2);
    let upper = r1.max(r2);
    ((lower + 1.0).floor() as i64, (upper - 1.0).ceil() as i64)
}

pub fn part_one(input: &str) -> Option<i64> {
    let (times, dists) = parse(input);
    let mut prod = 1;
    for i in 0..times.len() {
        let (lower, upper) = get_roots(times[i], dists[i]);
        prod *= (upper - lower) + 1;
    }
    Some(prod)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (time, dist) = parse2(input);
    let (lower, upper) = get_roots(time, dist);
    Some(upper - lower + 1)
}

advent_of_code::main!(6);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 6));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 6));
        assert_eq!(result, Some(71503));
    }
}
