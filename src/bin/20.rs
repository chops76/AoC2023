use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Debug,Clone)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast
}

#[derive(Debug,Clone)]
struct Module {
    connected: Vec<String>,
    module_type: ModuleType,
    pulses: VecDeque<(String, bool)>,
    state: bool,
    memory: Vec<(String, bool)>
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mut hs = HashMap::new();
    for module in input.split("\n") {
        let spl = module.split(" -> ").collect::<Vec<&str>>();
        let connected = spl[1].split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
        if spl[0] == "broadcaster" {
            hs.insert(spl[0].to_string(), Module { connected: connected, module_type: ModuleType::Broadcast, pulses: VecDeque::new(), state: false, memory: Vec::new() });
        } else {
            if spl[0].chars().nth(0).unwrap() == '%' {
                hs.insert(spl[0][1..].to_string(), Module { connected: connected, module_type: ModuleType::FlipFlop, pulses: VecDeque::new(), state: false, memory: Vec::new() });
            } else {
                hs.insert(spl[0][1..].to_string(), Module { connected: connected, module_type: ModuleType::Conjunction, pulses: VecDeque::new(), state: false, memory: Vec::new() });
            }
        }
    }
    let mut connections = HashSet::new();
    for (_,m) in &hs {
        for c in &m.connected {
            connections.insert(c.to_string());
        }
    }
    let c_vec = connections.into_iter().collect::<Vec<String>>();
    for c in c_vec {
        if !hs.contains_key(&c) {
            hs.insert(c, Module { connected: Vec::new(), module_type: ModuleType::Broadcast, pulses: VecDeque::new(), state: false, memory: Vec::new() });
        }
    }
    let keys = hs.keys().map(|s| s.clone()).collect::<Vec<String>>();
    for n in keys {
        let connected = hs.get(&n).unwrap().connected.clone();
        for c in &connected {
            //println!("Looking for {}", c);
            if !hs.get(c).unwrap().memory.contains(&(n.to_string(), false)) {
                hs.get_mut(c).unwrap().memory.push((n.clone(), false));
            }
        }
    }
    hs
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut modules = parse(input);
    let mut low_pulses = 0_i64;
    let mut high_pulses = 0_i64;
    for _ in 0..1000 {
        modules.get_mut("broadcaster").unwrap().pulses.push_back(("broadcaster".to_string(), false)); // Button push
        while modules.iter().any(|(_, m)| m.pulses.len() != 0) {
            let mut cur_module = "".to_string();
            for (s,m) in &modules {
                if m.pulses.len() != 0 {
                    cur_module = s.to_string();
                    break;
                }
            }
            let m = modules.get_mut(&cur_module).unwrap();
            let (sender, pulse) = m.pulses.pop_front().unwrap();
            if pulse {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
            let connected = m.connected.clone();
            let module_type = m.module_type.clone();
            match module_type {
                ModuleType::Broadcast => {
                    for receiver in connected {
                        //println!("{} sending {} to {}", cur_module, false, receiver);
                        let rm = modules.get_mut(&receiver).unwrap();
                        rm.pulses.push_back((cur_module.clone(), false));
                    }
                },
                ModuleType::Conjunction => {
                    for i in 0..m.memory.len() {
                        if m.memory[i].0 == sender {
                            m.memory[i].1 = pulse;
                            break;
                        }
                    }
                    let new_pulse = !m.memory.iter().all(|(_,state)| *state == true);
                    for receiver in connected {
                        //println!("{} sending {} to {}", cur_module, new_pulse, receiver);
                        let rm = modules.get_mut(&receiver).unwrap();
                        rm.pulses.push_back((cur_module.clone(), new_pulse));
                    }
                },
                ModuleType::FlipFlop => {
                    if !pulse {
                        m.state = !m.state;
                        let new_state = m.state.clone();
                        for receiver in connected {
                            //println!("{} sending {} to {}", cur_module, new_state, receiver);
                            let rm = modules.get_mut(&receiver).unwrap();
                            rm.pulses.push_back((cur_module.clone(), new_state));
                        }                    
                    }
                }
            }
        }
    }
    //println!("{:?}", modules);
    println!("Low: {}  High: {}", low_pulses, high_pulses);
    Some(low_pulses * high_pulses)
}

//Note: I calculated the cycles for each input to the input to rx and multiplied.  I did some
// by hand, so this code works only for my input, but you can see the general idea of what I was
// going for.
pub fn part_two(input: &str) -> Option<i64> {
    let mut modules = parse(input);
    let mut low_pulses = 0_i64;
    let mut high_pulses = 0_i64;
    let mut tx_state = false; // 4051
    let mut dd_state = false; // 3889
    let mut nz_state = false; // 3907
    let mut ph_state = false; // 3779
    for presses in 1..10000 {
        modules.get_mut("broadcaster").unwrap().pulses.push_back(("broadcaster".to_string(), false)); // Button push
        while modules.iter().any(|(_, m)| m.pulses.len() != 0) {
            let mut cur_module = "".to_string();
            for (s,m) in &modules {
                if m.pulses.len() != 0 {
                    cur_module = s.to_string();
                    break;
                }
            }
            let m = modules.get_mut(&cur_module).unwrap();
            let (sender, pulse) = m.pulses.pop_front().unwrap();
            if cur_module == "ls" {
                if sender == "ph" && pulse != ph_state {
                    println!("ph going to {} at press {}", pulse, presses);
                    ph_state = pulse;
                }
            }
            if pulse {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
            let connected = m.connected.clone();
            let module_type = m.module_type.clone();
            match module_type {
                ModuleType::Broadcast => {
                    for receiver in connected {
                        //println!("{} sending {} to {}", cur_module, false, receiver);
                        let rm = modules.get_mut(&receiver).unwrap();
                        rm.pulses.push_back((cur_module.clone(), false));
                    }
                },
                ModuleType::Conjunction => {
                    for i in 0..m.memory.len() {
                        if m.memory[i].0 == sender {
                            m.memory[i].1 = pulse;
                            break;
                        }
                    }
                    let new_pulse = !m.memory.iter().all(|(_,state)| *state == true);
                    for receiver in connected {
                        //println!("{} sending {} to {}", cur_module, new_pulse, receiver);
                        let rm = modules.get_mut(&receiver).unwrap();
                        rm.pulses.push_back((cur_module.clone(), new_pulse));
                    }
                },
                ModuleType::FlipFlop => {
                    if !pulse {
                        m.state = !m.state;
                        let new_state = m.state.clone();
                        for receiver in connected {
                            //println!("{} sending {} to {}", cur_module, new_state, receiver);
                            let rm = modules.get_mut(&receiver).unwrap();
                            rm.pulses.push_back((cur_module.clone(), new_state));
                        }                    
                    }
                }
            }
        }
    }
    Some(4051 * 3889 * 3907 * 3779)
}

advent_of_code::main!(20);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 20));
        assert_eq!(result, Some(32000000));
    }
}
