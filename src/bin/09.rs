fn parse(input: &str) -> Vec<Vec<i64>> {
    let spl = input.split("\n").collect::<Vec<&str>>();
    spl.into_iter().map(|s| s.split_ascii_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>()).collect::<Vec<Vec<i64>>>()
}

fn calc_next(seq: &Vec<i64>) -> i64 {
    let mut seqs = Vec::new();
    seqs.push(seq.clone());
    while !seqs[seqs.len() - 1].iter().all(|v| *v == 0) {
        let mut tmp_vec = Vec::new();
        let cur = seqs.len() - 1;
        for i in 0..seqs[cur].len() - 1 {
            tmp_vec.push(seqs[cur][i+1] - seqs[cur][i]);
        }
        seqs.push(tmp_vec);
    }
    let mut val = 0;
    for i in (0..seqs.len()).rev() {
        val += seqs[i][seqs[i].len() - 1];
    }
    val
}

fn calc_prev(seq: &Vec<i64>) -> i64 {
    let mut seqs = Vec::new();
    seqs.push(seq.clone());
    while !seqs[seqs.len() - 1].iter().all(|v| *v == 0) {
        let mut tmp_vec = Vec::new();
        let cur = seqs.len() - 1;
        for i in 0..seqs[cur].len() - 1 {
            tmp_vec.push(seqs[cur][i+1] - seqs[cur][i]);
        }
        seqs.push(tmp_vec);
    }
    let mut val = 0;
    for i in (0..seqs.len()).rev() {
        val = seqs[i][0] - val;
    }
    val
}

pub fn part_one(input: &str) -> Option<i64> {
    let seqs = parse(input);
    Some(seqs.iter().map(|s| calc_next(s)).sum())   
}

pub fn part_two(input: &str) -> Option<i64> {
    let seqs = parse(input);
    Some(seqs.iter().map(|s| calc_prev(s)).sum())   
}

advent_of_code::main!(9);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 9));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 9));
        assert_eq!(result, Some(2));
    }
}
