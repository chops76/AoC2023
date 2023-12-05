use std::collections::VecDeque;

fn parse(input: &str) -> (Vec<i64>, Vec<Vec<(i64, i64, i64)>>) {
    let spl = input.split("\n\n").collect::<Vec<&str>>();
    let seed_spl = spl[0].split(": ").collect::<Vec<&str>>();
    let seeds = seed_spl[1].split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut conversions = Vec::new();
    for i in 1..spl.len() {
        let range_spl = spl[i].split(":").collect::<Vec<&str>>();
        let mut range_vec = Vec::new();
        for range in range_spl[1].trim().split("\n") {
            let vals = range.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            range_vec.push((vals[0], vals[1], vals[2]));
        }
         conversions.push(range_vec);
    }

    (seeds, conversions)
}

fn get_location(seed: i64, conversions: &Vec<Vec<(i64, i64, i64)>>) -> i64 {
    let mut current = seed;
    for c in conversions {
        for r in c {
            if current >= r.1 && current < r.1 + r.2 {
                current += r.0 - r.1;
                break;
            }
        }
    }
    current
}

pub fn part_one(input: &str) -> Option<i64> {
    let (seeds, conversions) = parse(input);
    let mut lowest = i64::MAX;
    for seed in seeds {
        lowest = lowest.min(get_location(seed, &conversions));
    }
    Some(lowest)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (seeds, conversions) = parse(input);

    let mut ranges = VecDeque::new();
    for i in 0..(seeds.len() / 2) {
        ranges.push_back((seeds[2 * i],seeds[2 * i] + seeds[2 * i + 1] - 1));
    }
    for conversion in &conversions {
        let mut new_ranges = VecDeque::new();
        while let Some(range) = ranges.pop_front() {
            let mut found = false;
            for c in conversion {
                let conversion_start = c.1;
                let conversion_end = c.1 + c.2;
                if range.0 >= conversion_start && range.0 < conversion_end {
                    found = true;
                    if range.1 < conversion_end {
                        new_ranges.push_back((range.0 + c.0 - c.1, range.1 + c.0 - c.1));
                    } else {
                        new_ranges.push_back((range.0 + c.0 - c.1, conversion_end - 1 + c.0 - c.1));
                        ranges.push_back((conversion_end, range.1));
                    }
                    break;
                }
            }
            if !found {
                let mut next_lowest = i64::MAX;
                for c in &conversions[1] {
                    let conversion_start = c.1;
                    if conversion_start > range.0 {
                        next_lowest = next_lowest.max(conversion_start);
                    }
                }
                if next_lowest == i64::MAX || next_lowest > range.1 {
                    new_ranges.push_back((range.0, range.1));
                } else {
                    // This case was never actually hit by my input
                    new_ranges.push_back((range.0, next_lowest - 1));
                    ranges.push_back((next_lowest, range.1));
                }
            }
        }
        ranges = new_ranges;
    }
    let lowest = ranges.iter().map(|(v,_)| *v).min().unwrap();
    Some(lowest)
}

advent_of_code::main!(5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(46));
    }
}
