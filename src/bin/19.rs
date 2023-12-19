use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug,Clone)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64
}

#[derive(Debug)]
enum Operation {
    Less(char, i64),
    Greater(char, i64),
    Always
}

#[derive(Debug,Clone,PartialEq)]
enum Verdict {
    Accept,
    Reject,
    Workflow(String)
}

#[derive(Debug)]
struct WorkFlow {
    op: Operation,
    verdict: Verdict
}

fn parse(input: &str) -> (HashMap<String, Vec<WorkFlow>>, Vec<Part>) {
    let spl = input.split("\n\n").collect::<Vec<&str>>();
    let mut hm = HashMap::new();
    for w_str in spl[0].split("\n") {
        let w_spl = w_str.split("{").collect::<Vec<&str>>();
        let mut wfs = Vec::new();
        for wf in w_spl[1][..w_spl[1].len() - 1].split(",").collect::<Vec<&str>>() {
            if wf == "A" {
                wfs.push(WorkFlow { op: Operation::Always, verdict: Verdict::Accept });
            } else if wf == "R" {
                wfs.push(WorkFlow { op: Operation::Always, verdict: Verdict::Reject });
            } else if !wf.contains(":") {
                wfs.push(WorkFlow { op: Operation::Always, verdict: Verdict::Workflow(wf.to_string())});
            } else {
                let wf_sp = wf.split(":").collect::<Vec<&str>>();
                let c = wf_sp[0].chars().nth(0).unwrap();
                let val = wf_sp[0][2..].parse().unwrap();
                let op;
                if wf_sp[0].chars().nth(1) == Some('>') {
                    op = Operation::Greater(c, val);
                } else {
                    op = Operation::Less(c, val);
                }
                let verdict = match wf_sp[1] {
                    "A" => Verdict::Accept,
                    "R" => Verdict::Reject,
                    _ => Verdict::Workflow(wf_sp[1].to_string())
                };
                wfs.push(WorkFlow { op: op, verdict: verdict });
            }
        }
        hm.insert(w_spl[0].to_string(), wfs);
    }
    let mut parts = Vec::new();
    for p_str in spl[1].split("\n") {
        let rating_strings = p_str[1..p_str.len()-1].split(",").collect::<Vec<&str>>();
        parts.push(Part { x: rating_strings[0][2..].parse().unwrap(),
                          m: rating_strings[1][2..].parse().unwrap(),
                          a: rating_strings[2][2..].parse().unwrap(),
                          s: rating_strings[3][2..].parse().unwrap(), });
    }
    (hm, parts)
}

fn process_workflow(wf: &Vec<WorkFlow>, part: &Part) -> Verdict {
    for w in wf {
        match w.op {
            Operation::Always => {
                return w.verdict.clone();
            },
            Operation::Greater(c, test_val) => {
                let val = match c { 'x' => part.x, 'm' => part.m, 'a' => part.a, _ => part.s };
                if val > test_val {
                    return w.verdict.clone();
                }
            }
            Operation::Less(c, test_val) => {
                let val = match c { 'x' => part.x, 'm' => part.m, 'a' => part.a, _ => part.s };
                if val < test_val {
                    return w.verdict.clone();
                }
            }
        }
    }
    println!("ERROR: Finished workflow");
    Verdict::Reject
}

pub fn part_one(input: &str) -> Option<i64> {
    let (wf, parts) = parse(input);
    let mut sum = 0;
    for part in parts {
        let mut verdict = process_workflow(&wf["in"], &part);
        while let Verdict::Workflow(route) = verdict {
            verdict = process_workflow(&wf[&route], &part);
        }
        if verdict == Verdict::Accept {
            sum += part.x;
            sum += part.m;
            sum += part.a;
            sum += part.s;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (wf, _) = parse(input);
    let min = Part { x: 1, m: 1, a: 1, s: 1 };
    let max = Part { x: 4000, m: 4000, a: 4000, s: 4000 };
    let path: Vec<String> = Vec::new();
    let mut queue = VecDeque::new();
    let mut accepted = Vec::new();
    queue.push_back((min, max, path, "in".to_string()));
    while let Some((mut min, mut max, path, wf_name)) = queue.pop_front() {
        if path.contains(&wf_name) {
            continue;
        }
        let mut new_path = path.clone();
        new_path.push(wf_name.clone());
        for w in &wf[&wf_name] {
            match w.op {
                Operation::Always => {
                    if w.verdict == Verdict::Accept {
                        accepted.push((min.clone(), max.clone(), new_path.clone()));
                    } else if let Verdict::Workflow(new_wf) = &w.verdict {
                        queue.push_back((min.clone(), max.clone(), new_path.clone(), new_wf.to_string()));
                    }
                },
                Operation::Less(c, test_val) => {
                    let val_min = match c { 'x' => min.x, 'm' => min.m, 'a' => min.a, _ => min.s };
                    let val_max = match c { 'x' => max.x, 'm' => max.m, 'a' => max.a, _ => max.s };
                    if val_min < test_val && val_max >= test_val {
                        let mut new_max = max.clone();
                        let vr = match c { 'x' => &mut new_max.x, 'm' => &mut new_max.m, 'a' => &mut new_max.a, _ => &mut new_max.s };
                        *vr = test_val - 1;
                        if w.verdict == Verdict::Accept {
                            accepted.push((min.clone(), new_max.clone(), new_path.clone()));
                        }
                        if let Verdict::Workflow(new_wf) = &w.verdict {
                            queue.push_back((min.clone(), new_max, new_path.clone(), new_wf.to_string()));
                        }
                    } else if val_max < test_val {
                        if w.verdict == Verdict::Accept {
                            accepted.push((min.clone(), max.clone(), new_path.clone()));
                        }
                        if let Verdict::Workflow(new_wf) = &w.verdict {
                            queue.push_back((min.clone(), max.clone(), new_path.clone(), new_wf.to_string()));
                        } 
                        // We just sent all values down this path, don't continue workflow
                        break;                       
                    }
                    if val_min < test_val {
                        let vr = match c { 'x' => &mut min.x, 'm' => &mut min.m, 'a' => &mut min.a, _ => &mut min.s };
                        *vr = test_val;
                    }
                }
                Operation::Greater(c, test_val) => {
                    let val_min = match c { 'x' => min.x, 'm' => min.m, 'a' => min.a, _ => min.s };
                    let val_max = match c { 'x' => max.x, 'm' => max.m, 'a' => max.a, _ => max.s };
                    if val_max > test_val && val_min <= test_val {
                        let mut new_min = min.clone();
                        let vr = match c { 'x' => &mut new_min.x, 'm' => &mut new_min.m, 'a' => &mut new_min.a, _ => &mut new_min.s };
                        *vr = test_val + 1;
                        if w.verdict == Verdict::Accept {
                            accepted.push((new_min.clone(), max.clone(), new_path.clone()));
                        }
                        if let Verdict::Workflow(new_wf) = &w.verdict {
                            queue.push_back((new_min, max.clone(), new_path.clone(), new_wf.to_string()));
                        }
                    } else if val_min > test_val {
                        if w.verdict == Verdict::Accept {
                            accepted.push((min.clone(), max.clone(), new_path.clone()));
                        }
                        if let Verdict::Workflow(new_wf) = &w.verdict {
                            queue.push_back((min.clone(), max.clone(), new_path.clone(), new_wf.to_string()));
                        } 
                        // We just sent all values down this path, don't continue workflow
                        break;                       
                    }
                    if val_max > test_val {
                        let vr = match c { 'x' => &mut max.x, 'm' => &mut max.m, 'a' => &mut max.a, _ => &mut max.s };
                        *vr = test_val;
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for a in accepted {
        sum += (a.1.x - a.0.x + 1) * (a.1.m - a.0.m + 1) * (a.1.a - a.0.a + 1) * (a.1.s - a.0.s + 1);
    }
    Some(sum)
}

advent_of_code::main!(19);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 19));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 19));
        assert_eq!(result, Some(167409079868000));
    }
}
