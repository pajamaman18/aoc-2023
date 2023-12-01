use std::fmt::format;
use std::fs;
use std::io::BufRead;
use std::ops::Not;

fn main(){
    let file_path = "/home/pjman/personal-projects/aoc-2023-rust/input";
    println!("file path: {}", file_path);

    // let Ok(contents) = fs::read_to_string(file_path) else {todo!()};
    // let out = part2(contents.lines().map(|s|s.to_string()).collect::<Vec<String>>());
    // println!("{}",out)
    println!("day 1 part 1: {}",day1_1());
    println!("day 1 part 2: {}",day1_2());
}

fn day1_1() -> u32{
    let file_path = "/home/pjman/personal-projects/aoc-2023-rust/input";
    println!("file path: {}", file_path);

    if let Ok(contents) = fs::read_to_string(file_path) {
        let sum: u32 = contents
            .lines()
            .map(|l| l.to_string().chars().enumerate().filter_map(|(a,b)| {
                if b.is_digit(10){
                    Some(b)
                }else{
                    None
                }
            }).collect::<String>())
            .map(|nums| nums.chars().nth(0).unwrap() as u32 *10 + nums.chars().last().unwrap() as u32)
            .sum();
        return sum;
    }
    0
}

fn day1_2() -> u32 {
    let file_path = "/home/pjman/personal-projects/aoc-2023-rust/input";
    println!("file path: {}", file_path);

    if let Ok(contents) = fs::read_to_string(file_path) {
        let sum: u32 = contents
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let modified_line = change_vals2(line.to_string());
                let first = modified_line.chars().nth(0).unwrap();
                let last = modified_line.chars().last().unwrap();
                format!("{}{}",first,last).parse::<u32>().unwrap()
            })
            .sum();

        return sum;
    }
    0
}

fn change_vals2(mut s: String) -> String {
    let units = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    // let values = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut new_string = "".to_string();
    let mut nums:Vec<u32> = vec![];
    for (index, letter) in s.chars().enumerate(){
        if letter.is_digit(10){
           nums.push(letter.to_string().parse::<u32>().unwrap())
        }else {
            let ind = (0..units.len())
                .map(|i| s[index..].find(units[i]))
                .map(|value| {
                    if let Some(v) = value{
                        v
                    }else{
                        1000000000000000
                    }
                });
                for (index, a) in ind.enumerate(){
                    if a == 0{
                        nums.push(index as u32 +1);
                    }
                }
        }
    }
    format!("{}{}",nums.iter().nth(0).unwrap(), nums.last().unwrap())

    // let mut running = true;
    // while running {
    //     let mut index = 100000000;
    //     let mut value = 10000;
    //     for i in 0..units.len() {
    //         let place = s.find(units[i]);
    //         if let Some(p) = place {
    //             if p <= index {
    //                 index = p;
    //                 value = i;
    //             }
    //         }
    //     }
    //     if value < 10 {
    //         s = s.replacen(units[value], values[value], 1);
    //         new_string += &s[new_string.len()..index+1].to_string();
    //         s = s[1..].to_string();
    //     }else{
    //         new_string = s.to_string();
    //         running = false
    //     }
    // }
    // new_string.retain(|c| c.is_digit(10));
    // new_string.to_string()
}
