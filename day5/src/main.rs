use std::collections::HashSet;
use std::fs;
use std::ops::Index;

fn main() {
    println!("day 5 part 1: {}", day5_1());
    println!("day 5 part 2: {}", day5_2());
}
fn day5_2() -> u64{
    let parse = parse_input();
    let seeds = parse.index(0).index(0).clone();
    let mut new_vals = &mut vec![];
    for i in 0..(seeds.len()/2){
        new_vals.push((seeds[2*i], seeds[2*i]+seeds[2*i+1]-1));
    }
    let maps = parse.iter().skip(1);
    let mut mapped_vals= vec![];
    for map in maps{
        mapped_vals = vec![];
        while !new_vals.is_empty(){
            let (mut b, mut e) = new_vals.pop().unwrap();
            println!("\nbegin: {}, end {}\n",b,e);
            for mapping in map{
                let m = mapping.clone();
                let start = mapping[1];
                let end = start + mapping[2]-1;
                println!("({}:{})",b,e);
                if b >= start && e <= end{ //in between
                    let mapped_range = (translate_seed(b,m.clone()).unwrap(),translate_seed(e,m).unwrap());
                    mapped_vals.push(mapped_range);
                    (b,e) = (0,0);
                    break; // range has been consumed
                }else if b >= start && b <= end && e > end { // spills out on right side
                    let mapped_range = (translate_seed(b,m.clone()).unwrap(),translate_seed(end,m).unwrap());
                    mapped_vals.push(mapped_range);
                    (b,e) = (end+1,e);
                }else if b < start && e <= end && e >= start{ // spills out on left side
                    let mapped_range = (translate_seed(start,m.clone()).unwrap(),translate_seed(e,m).unwrap());
                    mapped_vals.push(mapped_range);
                    (b,e) = (b,start-1);
                }
            }
            if b+e != 0{
                mapped_vals.push((b,e));
            }
        }
        for val in &mapped_vals{
            new_vals.push(*val);
        }
    }
    mapped_vals.iter().map(|(a,_)|*a).min().unwrap()
}

fn day5_1() -> u64{
    let parse = parse_input();
    let seeds = parse.index(0).index(0).clone();
    let only_maps = parse.into_iter().skip(1).collect();
    calc_min_seed(seeds,only_maps)
}
fn calc_min_seed(seeds: Vec<u64>, maps:Vec<Vec<Vec<u64>>>) -> u64{
    let mut ind = 0;
    seeds.iter().map(|&seed| {
        println!("{}/{}",ind,seeds.len());
        ind+=1;
        let mut curr_value = seed;
        for map in maps.iter() {
            let map_val =
                map.iter()
                    .filter_map(|onemap| {
                        translate_seed(curr_value,onemap.clone())
                    }).collect::<Vec<u64>>();
            if map_val.len() == 1 {
                curr_value = *map_val.index(0);
            }
        }
        curr_value
    }).min().unwrap()
}

fn translate_seed(curr_value:u64, trans: Vec<u64>) -> Option<u64>{
    let source = 1;
    let dest = 0;
    let range = 2;
    println!("val: {} in [{} ; {}]",curr_value,trans[source],trans[source]+trans[range]);
    if (trans[source]..(trans[source] + trans[range])).contains(&curr_value) {
        let offset = curr_value - trans[source];
        Some(trans[dest] + offset)
    } else {
        None
    }
}

// all except first vec should be grouped in blocks of 3
fn parse_input() -> Vec<Vec<Vec<u64>>> {
    let file_path = "/home/pjman/personal-projects/aoc-2023-rust/input";
    if let Ok(mut contents) = fs::read_to_string(file_path) {
        contents.insert(7usize,'\n');
        // let split_lines = contents.lines()
        return contents.split("\n\n")
            .map(|a|a.lines().skip(1)
                .map(|line| line.to_string().split(" ").map(|a| a.parse::<u64>().unwrap()).collect::<Vec<u64>>()).collect::<Vec<Vec<u64>>>()
            )
            .collect::<Vec<Vec<Vec<u64>>>>();
    }
    vec![]
}