use std::fmt::format;
use std::fs;
use std::io::BufRead;
use std::ops::Not;

fn main(){
    day1_2()
}

fn day1_1(){

}

// fn day1_2(){
//     let file_path = "/home/pjman/personal-projects/aoc-2023-rust/input";
//     println!("file path:{}", file_path);
//     let contents = fs::read_to_string(file_path).expect("no file found");
//     let vec: Vec<String> = contents.split('\n').filter_map(|w|w.parse::<String>().ok()).collect();
//     // let vec = split.collect::<Vec<&str>>();
//     // println!("{:?}", vec);
//     let mut seperated_ints: Vec<u32> = vec![];
//     let mut sum = 0;
//     for line in vec{
//         sum += val(&line).;
//     }
//     // let sum = vec.iter().map(
//     //     |line| {
//     //         val(line).unwrap()
//     //
//     //     })
//     //     .sum::<u32>();
//     print!("{}",sum)
// }
// fn val(s:&str) -> Result<u32,&str>{
//     let _units = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
//     let l = s.len();
//     let first = (0..l)
//         .flat_map(|i| digits(&s[i..]))
//         .nth(0)
//         .ok_or("death")
//         .unwrap();
//     let last = (0..l)
//         .rev()
//         .flat_map(|i| digits(&s[i..]))
//         .nth(0)
//         .ok_or("death")
//         .unwrap();
//     Ok(first*10+last)
// }
//
// fn digits(s:&str) -> Option<u32>{
//     if let Ok(x) = s[0..1].parse() {
//         return Some(x);
//     }
//
//     let numbers = [
//         "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
//     ];
//
//     for (idx, num) in numbers.iter().enumerate() {
//         if s.starts_with(num) {
//             return Some(idx as u32 + 1);
//         }
//     }
//
//     None
// }

fn day1_2(){
    let file_path = "/home/pjman/personal-projects/aoc-2023-rust/input";
    println!("file path: {}", file_path);

    if let Ok(contents) = fs::read_to_string(file_path) {
        let sum: u32 = contents
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let modified_line = change_vals2(line.to_string());
                // let digits: String = modified_line.chars().filter(|c| c.is_digit(10)).collect();
                // println!("{}",digits);
                // let d = digits.parse::<u32>().unwrap_or(0)
                let first = modified_line.chars().nth(0).unwrap();
                let last = modified_line.chars().last().unwrap();
                format!("{}{}",first,last).parse::<u32>().unwrap()
            })
            .sum();

        println!("Sum of calibration values: {}", sum);
    } else {
        eprintln!("Error reading the file");
    }
}

fn change_vals2(mut s: String) -> String {
    let units = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let values = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    for (unit, value) in units.iter().zip(values.iter()) {
        s = s.replace(unit, value);
    }
    s.retain(|c| c.is_digit(10));
    s
}

fn old(){
    let file_path = "/home/pjman/personal-projects/aoc-2023-rust/input";
    println!("file path:{}", file_path);
    let contents = fs::read_to_string(file_path).expect("no file found");
    let vec: Vec<&str> = contents.lines().collect();
    // let vec = split.collect::<Vec<&str>>();
    // println!("{:?}", vec);
    let mut seperated_ints: Vec<u32> = vec![];
    for cali in vec{
        let mut ints: Vec<char> = vec![];
        print!("{}",cali);
        let mut changed_cali = cali.to_string();
        changed_cali = change_vals(changed_cali);
        print!(" -> {}",changed_cali);
        for (_,c) in changed_cali.chars().enumerate(){
            if c.is_ascii_digit() {
                ints.push(c)
            }
        }
        let str;
        str = format!("{}{}", ints[0],ints[ints.len()-1]);
        println!(" -> {}", str);
        seperated_ints.push(str.parse::<u32>().unwrap())

    }
    let mut output = 0;
    for i in seperated_ints{
        output += i
    }
    println!("{}", output);
}

fn change_vals(mut s: String) -> String {
    let _units = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let _values = vec!["1","2","3","4","5","6","7","8","9"];
    let mut running = true;
    while running {
        let mut index = 100000000;
        let mut value = 10000;
        for i in 0.._units.len(){
            let place = s.find(_units[i]);
            if let Some(p) = place{
                if p <= index{
                    index = place.unwrap();
                    value = i;
                }
            }
        }
        if value < 100{
            let replacement = _values[value];
            let start = index;
            let end = index + _units[value].len();
            s.replace_range(start..end, replacement);
        }else{
            running = false;
        }
    }
    s.retain(|c| c.is_digit(10) || c.is_whitespace().not());  // Keep only digits and remove spaces
    s
}