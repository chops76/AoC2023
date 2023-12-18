fn parse(input: &str) -> Vec<(char, usize, String)> {
    let mut ret_vec = Vec::new();
    for s in input.split("\n") {
        let spl = s.split(" ").collect::<Vec<&str>>();
        ret_vec.push((spl[0].chars().next().unwrap(), spl[1].parse::<usize>().unwrap(), spl[2].to_string()));
    }
    ret_vec
}

fn pick_area(vertices: &Vec<(i64, i64)>) -> f64 {
    let mut area = 0.0;
    let mut perimeter = 0;

    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        area += (vertices[i].0 * vertices[j].1) as f64;
        area -= (vertices[j].0 * vertices[i].1) as f64;
        perimeter += (vertices[i].0 - vertices[j].0).abs() + (vertices[i].1 - vertices[j].1).abs();
    }
    (area.abs() / 2.0) + perimeter as f64 / 2.0 + 1.0
}

pub fn part_one(input: &str) -> Option<u64> {
    let commands = parse(input);
    let mut points = Vec::new();
    let mut x = 0;
    let mut y = 0;
    for (inst, dist, color) in &commands {
        let xd = match *inst { 'U' => 0, 'D' => 0, 'L' => -1, _ => 1 };
        let yd = match *inst { 'U' => -1, 'D' => 1, 'L' => 0, _ => 0 };
        x += xd * *dist as i64;
        y += yd * *dist as i64;
        points.push((x, y));
    }
    Some(pick_area(&points) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let commands = parse(input);
    let mut points = Vec::new();
    let mut x = 0;
    let mut y = 0;
    for (_, _, color) in &commands {
        let len_str = color[2..7].to_string();
        let len = i64::from_str_radix(&len_str, 16).unwrap();
        let cmd = color.chars().nth(7).unwrap();
        let xd = match cmd { '3' => 0, '1' => 0, '2' => -1, _ => 1 };
        let yd = match cmd { '3' => -1, '1' => 1, '2' => 0, _ => 0 };
        x += xd * len;
        y += yd * len;
        points.push((x, y));
    }
    Some(pick_area(&points) as u64)
}

advent_of_code::main!(18);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 18));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 18));
        assert_eq!(result, Some(952408144115));
    }
}
