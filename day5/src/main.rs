use std::collections::HashSet;
use std::fs;
use std::ops::Index;

fn main() {
    // for v in parse_input(){
    //     println!("{:?}",v);
    // }
    println!("day 5 part 1: {}", day5_1());
    println!("day 5 part 2: {}", day5_2());
}
fn day5_2() -> u64{
    let parse = parse_input();
    let seeds = parse.index(0).index(0).clone();
    let new_seeds = build_seed_range(seeds);
    let only_maps = parse.into_iter().skip(1).collect();

    calc_min_seed(new_seeds,only_maps)
}

fn build_seed_range(seed_range:Vec<u64>) -> Vec<u64>{
    let mut new_seeds = HashSet::new();
    for i in 0..(seed_range.len()/2){
        for s in seed_range[2*i]..(seed_range[2*i]+seed_range[2*i+1]) {
            new_seeds.push(s)
        }
    }
    new_seeds.iter().collect::<Vec<u64>>()
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
                        translate_seed(curr_value,onemap)
                    }).collect::<Vec<u64>>();
            if map_val.len() == 1 {
                curr_value = *map_val.index(0);
            }
        }
        curr_value
    }).min().unwrap()
}

fn translate_seed(curr_value:u64, trans: &Vec<u64>) -> Option<u64>{
    let source = 1;
    let dest = 0;
    let range = 2;
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