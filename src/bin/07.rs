use std::collections::HashMap;

fn get_rank(card: char) -> i64 {
    match card {
        '2' => { 1 },
        '3' => { 2 },
        '4' => { 3 },
        '5' => { 4 },
        '6' => { 5 },
        '7' => { 6 },
        '8' => { 7 },
        '9' => { 8 },
        'T' => { 9 },
        'J' => { 10 },
        'Q' => { 11 },
        'K' => { 12 },
        'A' => { 13 },
        _ => { 0 }
    }
}

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    let spl = input.split("\n").collect::<Vec<&str>>();
    let mut ret_vec = Vec::new();
    for s in spl {
        let hand_split = s.split_whitespace().collect::<Vec<&str>>();
        let bid = hand_split[1].parse().unwrap();
        let ranks = hand_split[0].chars().map(|c| get_rank(c)).collect::<Vec<i64>>();
        ret_vec.push((bid, ranks));
    }
    ret_vec
}

fn hand_score(hand: &Vec<i64>) -> i64 {
    let mut hist = HashMap::new();

    for v in hand {
        let entry = hist.entry(*v).or_insert(0);
        *entry += 1;
    }
    let mut counts = hist.values().map(|v| *v).collect::<Vec<usize>>();
    counts.sort();
    counts.reverse();
    if counts[0] == 5 {
        return 7;
    }
    match (counts[0], counts[1]) {
        (4, 1) => { 6 },
        (3, 2) => { 5 },
        (3, 1) => { 4 },
        (2, 2) => { 3 },
        (2, 1) => { 2 },
        _ => { 1 }
    }
}

fn hand_value(hand: &Vec<i64>) -> i64 {
    let mut value = 0;
    for v in hand {
        value *= 100;
        value += v;
    }
    let mut best_score = 0;
    for i in 1..=13 {
        if i == 10 {
            continue;
        }
        let mut temp_hand = hand.clone();
        for j in 0..temp_hand.len() {
            if temp_hand[j] == 0 {
                temp_hand[j] = i;
            }
        }
        best_score = best_score.max(hand_score(&temp_hand));
    }
    best_score * 10000000000 + value
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut hands = parse(input);
    hands.sort_by_key(|(_,h)| hand_value(h));
    let mut sum = 0;
    for i in 0..hands.len() {
        sum += hands[i].0 * (i as i64 + 1);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut hands = parse(input);
    for j in 0..hands.len() {
        for i in 0..hands[j].1.len() {
            if hands[j].1[i] == 10 {
                hands[j].1[i] = 0;
            }
        }
    }
    hands.sort_by_key(|(_,h)| hand_value(h));
    let mut sum = 0;
    for i in 0..hands.len() {
        sum += hands[i].0 * (i as i64 + 1);
    }    
    Some(sum)
}

advent_of_code::main!(7);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 7));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 7));
        assert_eq!(result, Some(5905));
    }
}
