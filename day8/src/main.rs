use std::collections::HashMap;
use std::fs;

use num::integer::lcm;


fn main() {
    println!("day 8 part 1: {:?}", day8_1());
    println!("day 8 part 2: {:?}", day8_2());
}

fn day8_1() -> u32 {
    let (turns, hmap) = parse();
    let mut current_space = "AAA";
    let mut steps = 0;
    for turn in turns.chars().cycle() {
        if current_space.contains("ZZZ") {
            return steps;
        }
        steps += 1;
        let [lturn, rturn] = hmap.get(current_space).unwrap();
        match turn {
            'L' => current_space = lturn,
            'R' => current_space = rturn,
            _ => println!("direction error")
        }
    }
    0
}

fn day8_2() -> u64 {
    let mut cycles = get_cycle_lengths();
    let mut out = cycles.pop().unwrap();
    for c in cycles {
        out = lcm(c, out);
    }
    out
}

fn get_cycle_lengths() -> Vec<u64> {
    let (turns, hmap) = parse();
    let mut current_spaces = vec![];
    for key in hmap.keys() {
        if key.ends_with('A') {
            current_spaces.push(key.to_string());
        }
    }
    let mut passed_spaces = vec![];
    let mut cycles = vec![];
    let mut steps = 0;
    for turn in turns.chars().cycle() {
        if current_spaces.is_empty() {
            return cycles;
        }
        while !current_spaces.is_empty() {
            let current_space = current_spaces.pop().unwrap().to_string();
            let next_space;
            let [lturn, rturn] = hmap.get(&current_space).unwrap();
            match turn {
                'L' => next_space = lturn.to_string(),
                'R' => next_space = rturn.to_string(),
                _ => next_space = "".to_string()
            }
            passed_spaces.push(next_space)
        }
        steps += 1;
        for sp in passed_spaces {
            if !sp.ends_with('Z') {
                current_spaces.push(sp);
            } else {
                cycles.push(steps)
            }
        }

        passed_spaces = vec![];
    }
    return cycles;
}

fn parse() -> (String, HashMap<String, [String; 2]>) {
    let file_path = "../input";
    if let Ok(contents) = fs::read_to_string(file_path) {
        let mut hmap = HashMap::new();
        let split = contents.split("\n\n").collect::<Vec<&str>>();
        let turns = split[0].to_string();
        let map = split[1].to_string();
        for line in map.lines() {
            let s = line.split(' ').collect::<Vec<&str>>();
            // println!("{:?}",s);
            let starting_node = s[0].to_string();
            let left_node = s[2].chars().skip(1).collect::<String>().split(',').collect::<Vec<&str>>()[0].to_string();
            let right_node = s[3].split(')').collect::<Vec<&str>>()[0].to_string();
            let mut map: [String; 2] = ["".to_string(), "".to_string()];
            map[0] = left_node;
            map[1] = right_node;
            hmap.insert(starting_node, map);
        }

        return (turns.to_string(), hmap);
    }
    ("".to_string(), HashMap::new())
}