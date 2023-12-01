fn find_sum(input: &str) -> Option<u32> {
    let v = input.split_whitespace()
                           .map(|st| st.chars()
                                             .into_iter()
                                             .filter(|c| c.is_ascii_digit())
                                             .collect::<Vec<char>>())
                           .map(|d| d[0].to_digit(10).unwrap() * 10 + d[d.len() - 1].to_digit(10).unwrap())
                           .sum();
    Some(v)
}

pub fn part_one(input: &str) -> Option<u32> {
    find_sum(input)
}

fn fix_first(input: &str) -> String {
    let names = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let pos = names.iter().map(|n| input.find(n)).collect::<Vec<Option<usize>>>();
    let best = pos.into_iter().enumerate().filter(|v| v.1 != None).collect::<Vec<(usize, Option<usize>)>>();
    let mut ret_str = input.to_string();
    if best.len() != 0 {
        let b = best.into_iter().min_by_key(|v| v.1).unwrap();
        let p = b.1.unwrap();
        let v = b.0 + 1;
        let s = format!("{}", v);
        ret_str = ret_str.replacen(names[b.0], &s, 1);
    }

    ret_str
}

fn fix_last(input: &str) -> String {
    let names = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let pos = names.iter().map(|n| input.rfind(n)).collect::<Vec<Option<usize>>>();
    let best = pos.into_iter().enumerate().filter(|v| v.1 != None).collect::<Vec<(usize, Option<usize>)>>();
    let mut ret_str = input.to_string();
    if best.len() != 0 {
        let b = best.into_iter().max_by_key(|v| v.1).unwrap();
        let v = b.0 + 1;
        let s = format!("{}", v);
        ret_str = ret_str.replace(names[b.0], &s);
    }

    ret_str
}

pub fn part_two(input: &str) -> Option<u32> {
    let s = input.split_ascii_whitespace().collect::<Vec<&str>>();
    let mut sum = 0;
    for st in s {
        let first = fix_first(st);
        let last = fix_last(st);
        let first_digits = first.chars()
                                        .into_iter()
                                        .filter(|c| c.is_ascii_digit())
                                        .collect::<Vec<char>>();
        let last_digits = last.chars()
                                    .into_iter()
                                    .filter(|c| c.is_ascii_digit())
                                    .collect::<Vec<char>>();
        let v = first_digits[0].to_digit(10).unwrap() * 10 +
                last_digits[last_digits.len() - 1].to_digit(10).unwrap();
        sum += v;
    }
    Some(sum)
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(281));
    }
}
