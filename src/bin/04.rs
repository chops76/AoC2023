use std::collections::HashSet;

fn parse(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut ret_vec = Vec::new();
    for line in lines {
        let nums = line.split(": ").collect::<Vec<&str>>();
        let spl = nums[1].split(" | ").collect::<Vec<&str>>();
        let winning = spl[0].split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let mine = spl[1].split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        ret_vec.push((winning, mine));
    }

    ret_vec
}

fn num_matches(winning: &Vec<u32>, mine: &Vec<u32>) -> usize {
    let winning = winning.into_iter().collect::<HashSet<&u32>>();
    let mine = mine.into_iter().collect::<HashSet<&u32>>();
    let both = winning.intersection(&mine).collect::<HashSet<&&u32>>();
    both.len()
}

fn score(winning: &Vec<u32>, mine: &Vec<u32>) -> u32 {
    let num = num_matches(winning, mine);
    if num != 0 {
        return 2_u32.pow(num as u32 - 1);
    }
    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse(input);
    Some(cards.iter().map(|(w, m)| score(&w, &m)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse(input);
    let mut num_cards = vec![1; cards.len()];
    for i in 0..cards.len() {
        let matches = num_matches(&cards[i].0, &cards[i].1);
        for j in i+1..i+1+matches {
            num_cards[j] += num_cards[i];
        }
    }
    Some(num_cards.iter().sum())
}

advent_of_code::main!(4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(30));
    }
}
