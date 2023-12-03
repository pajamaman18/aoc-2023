use std::fs;
use std::ops::Index;

fn main() {
    println!("day 3 part 1: {}", day3_1());
    println!("day 3 part 2: {}", day3_2());
}

fn day3_2() -> u32{
    let (file, nodes) = parse_input(true);
    let matrix = file.lines().map(|s| s.to_string()).collect::<Vec<String>>();
    nodes.iter().map(|(x, y)| two_vals(&matrix, *x, *y)).sum()

}

fn two_vals(matrix: &Vec<String>, y: usize, x: usize) -> u32 {
    let mut prev_hit = false;
    let mut hits = 0;
    let mut temp = 1;
    for j in 0..3 {
        let t = matrix[y - 1usize + j].to_string();
        for i in 0..3 {
            if hits > 2{
                return 0;
            }
            if x + i < 141 && y + j < 141 {
                let c = t.index((x + i - 1usize)..(x + i));
                if "1234567890".contains(c) {
                    if !prev_hit {
                        let b = matrix[y - 1usize + j].to_string();
                        let first = x - 1usize + i - 2usize;
                        let mut last = x - 1usize + i + 2usize + 1usize;
                        if last > 140 {
                            last = 140
                        }
                        let substr: &str = b.index(first..last);
                        let value = glob_part_number_single(substr);
                        temp *= value;
                        hits += 1;
                        prev_hit = true;
                    }
                }else{
                    prev_hit = false;
                }
            }
        }
        prev_hit = false
    }
    if hits == 2{
        temp
    }else{
        0
    }
}

fn day3_1() -> u32 {
    // let length = 141;
    let (file, mut nodes) = parse_input(false);
    // nodes.sort();
    // let max_size = file.len();
    // let mut part_numbers = vec![];
    let matrix = file.lines().map(|s| s.to_string()).collect::<Vec<String>>();
    let mut size = 0;
    for (y, x) in nodes {
        let mut prev_hit = false;
        for j in 0..3 {
            let t = matrix[y - 1usize + j].to_string();
            for i in 0..3 {
                if x + i < 141 && y + j < 141 {
                    let c = t.index((x + i - 1usize)..(x + i));
                    if "1234567890".contains(c) {
                        if !prev_hit {
                            let b = matrix[y - 1usize + j].to_string();
                            let first = x - 1usize + i - 2usize;
                            let mut last = x - 1usize + i + 2usize + 1usize;
                            // let substr: &str = b.index((first - 1usize)..(last - 1usize));
                            // print!("{}   ",substr_o);
                            if last > 140 {
                                last = 140
                            }
                            // println!("{:?}",b.index(first..140));
                            let substr: &str = b.index(first..last);
                            // println!("{}",substr.clone());
                            let value = glob_part_number_single(substr);
                            size += value;
                            prev_hit = true;
                        }
                    } else {
                        prev_hit = false;
                    }
                }

                // if check_index < max_size as u32 && check_index > 0{
                //     let c:&str = file.index((((check_index-1) as usize)..(check_index as usize)));
                //     if "1234567890".contains(c) {
                //         if !prev_hit {
                //             part_numbers.push(check_index - 1);
                //             prev_hit = true;
                //         }
                //     }else {
                //         prev_hit = false;
                //     }
            }
            prev_hit = false;
        }
    }
    size
}
//     let _b = part_numbers.clone();
//     // part_numbers = part_numbers.iter().enumerate()
//     //     .filter_map(|(index, &val)| {
//     //         if index > 1 && (val-1 == *part_numbers.index(index - 1usize) || val-2 == *part_numbers.index(index - 1usize)){
//     //             None
//     //         }else{
//     //             Some(val)
//     //         } }).collect();
//     // print!("{:?} ", _b);
//     part_numbers.sort();
//     glob_part_number(file,part_numbers)
//
// }

fn glob_part_number_single(substr: &str) -> u32 {
    let val;
    if "0123456789".contains(substr.index(1usize..2usize)) {
        if "0123456789".contains(substr.index(0usize..1usize)) {
            val = substr[0usize..3usize].to_string();
        } else {
            val = substr[1usize..4usize].to_string();
        }
    } else {
        if "0123456789".contains(substr.index(3usize..4usize)) {
            val = substr[2usize..5usize].to_string();
        } else {
            val = substr[2usize..3usize].to_string();
        }
    }
    let filt_val = val.chars().filter(|&c| !".*+=-/&%$#@".contains(c)).collect::<String>();
    filt_val.parse::<u32>().unwrap()
}

fn glob_part_number(file: String, list: Vec<u32>) -> u32 {
    let mut numbers = vec![];
    for index in list {
        let i = index as usize;
        let substr: &str = &file[((i - 2) as usize)..((i + 3) as usize)];
        let val;
        if "0123456789".contains(substr.index(1usize..2usize)) {
            if "0123456789".contains(substr.index(0usize..1usize)) {
                val = substr[0usize..3usize].to_string()
            } else {
                val = substr[1usize..4usize].to_string()
            }
        } else {
            val = substr[2usize..5usize].to_string()
        }
        let filt_val = val.chars().filter(|&c| !".*+=-/&%$#@".contains(c)).collect::<String>();
        numbers.push(filt_val.parse::<u32>().unwrap());
        // let reg = Regex::new(r".*(\d\d\d|[.*+=\-/&%$#@]\d\d\.).*").expect("a");
        // for (_, [num]) in reg.captures_iter(substr).map(|c| c.extract()) {
        //     numbers.push(num.parse::<u32>().unwrap());
        // }
        // let a = substr.split('.')
        //     .filter(|&slice| slice.len() > 2)
        //     .collect::<Vec<&str>>();
    }

    numbers.iter().sum()
    // numbers.iter().sum()
    // let b = list.iter()

    //     .map(|&index| {
    //         let substr: &str = &file[((index - 2) as usize)..((index + 2) as usize)];
    //         let a = substr.split('.')
    //             .filter(|&slice| slice.len() > 2)
    //             .collect::<Vec<&str>>();
    //         println!("{:?}", a)
    //     });
}

fn parse_input(part2: bool) -> (String, Vec<(usize, usize)>) {
    let file_path = "/home/pjman/personal-projects/aoc-2023-rust/input";
    if let Ok(contents) = fs::read_to_string(file_path) {
        let split_lines = contents.lines();
        let mut other_coords = vec![];
        for (j, line) in split_lines.enumerate() {
            for (i, c) in line.to_string().chars().enumerate() {
                if "*+=-/&%$#@".contains(c) && !part2 {
                    other_coords.push((j, i));
                } else {
                    if "*".contains(c) {
                        other_coords.push((j, i));
                    }
                }
            }
        }
        // let line_parts = contents.clone()
        //     .chars()
        //     .enumerate()
        //     .filter_map(|(index,c)| {
        //         if "*+=-/&%$#@".contains(c){
        //             Some((index) as u32)
        //         } else {
        //             None
        //         }
        //     }).collect::<Vec<u32>>();

        return (contents, other_coords);
    }
    ("".to_string(), vec![])
    // for line in
}