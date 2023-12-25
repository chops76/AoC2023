fn parse(input: &str) -> Vec<((i64, i64, i64), (i64, i64, i64))> {
    let mut ret_vec = Vec::new();
    for s in input.split("\n").collect::<Vec<&str>>() {
        let spl = s.split(" @ ").collect::<Vec<&str>>();
        let pos = spl[0].split(", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<i64>>();
        let vel = spl[1].split(", ").map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();
        ret_vec.push(((pos[0], pos[1], pos[2]),(vel[0], vel[1], vel[2])));
    }
    ret_vec
}

pub fn part_one(input: &str) -> Option<u32> {
    let stones = parse(input);
    let mut sum = 0;
    for s1 in 0..stones.len() - 1 {
        for s2 in s1 + 1..stones.len() {
            let m0 = stones[s1].1.1 as f64 / stones[s1].1.0 as f64;
            let m1 = stones[s2].1.1 as f64 / stones[s2].1.0 as f64;
            let x0 = stones[s1].0.0 as f64;
            let x1 = stones[s2].0.0 as f64;
            let y0 = stones[s1].0.1 as f64;
            let y1 = stones[s2].0.1 as f64;
            if m0 == f64::INFINITY || m1 == f64::INFINITY {
                println!("Vertical?");
            }
            if stones[s1].1.0 == 0 || stones[s2].1.0 == 0 {
                println!("Vert??");
            }
            let x = (m0 * x0 - y0 - m1 * x1 + y1) / (m0 - m1);
            //println!("x-coord = {}", x);
            if x == f64::INFINITY {
                //println!("Parallel");
                continue;
            }
            if (x < x0 && stones[s1].1.0 > 0) || (x > x0 && stones[s1].1.0 < 0) {
                //println!("In the past A");
                continue;
            }
            if (x < x1 && stones[s2].1.0 > 0) || (x > x1 && stones[s2].1.0 < 0) {
                //println!("In the past B");
                continue;
            }
            let y = m0 * (x - x0) + y0;
            if x >= 200000000000000.0 && x <= 400000000000000.0 &&
               y >= 200000000000000.0 && y <= 400000000000000.0 {
                sum += 1;
               }
            //println!("point: ({},{})", x, y);
        }
    }
    //println!("{:?}", stones);
    Some(sum) // 27349, 13631
}

pub fn part_two(input: &str) -> Option<u32> {
    // I used Wolfram Alpha to solve a series of equations.  I couldn't get the Rust 
    // Z3 package to work.
    None
}

advent_of_code::main!(24);

