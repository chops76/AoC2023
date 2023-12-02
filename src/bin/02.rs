fn parse(input: &str) -> Vec<(usize, usize, usize)> {
    let mut ret_vec = Vec::new();
    let round_strings = input.split("\n").collect::<Vec<&str>>();
    for s in round_strings {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        let game_split = s.split(": ").collect::<Vec<&str>>();
        let grabs = game_split[1].split("; ").collect::<Vec<&str>>();
        for grab in grabs {
            let grab_split = grab.split(", ").collect::<Vec<&str>>();
            for color in grab_split {
                let color_split = color.split_whitespace().collect::<Vec<&str>>();
                let val = color_split[0].parse::<usize>().unwrap();
                match color_split[1] {
                    "blue" => { max_blue = max_blue.max(val) },
                    "red" => { max_red = max_red.max(val) },
                    "green" => { max_green = max_green.max(val) },
                    _ => { println!("Bad color") }
                }
            }
        }
        ret_vec.push((max_red,max_green,max_blue));
    }
    ret_vec
}

pub fn part_one(input: &str) -> Option<usize> {
    let rounds = parse(input);
    let mut sum = 0;
    for i in 0..rounds.len() {
        if rounds[i].0 <= 12 && rounds[i].1 <= 13 && rounds[i].2 <= 14 {
            sum += i + 1;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let rounds = parse(input);
    let val = rounds.iter().map(|(a, b, c)| a*b*c).sum();
    Some(val)
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(2286));
    }
}
