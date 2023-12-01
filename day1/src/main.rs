use std::fs;
use std::io::BufRead;

fn main(){
    let file_path = "/home/pjman/personal-projects/aoc-2023-rust/input";
    println!("file path: {}", file_path);

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

}
