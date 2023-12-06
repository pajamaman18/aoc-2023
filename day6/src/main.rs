use std::fs;
use std::iter::{zip};

fn main() {
    println!("day6 part 1: {}",day6_1());
    println!("day6 part 2: {}",day6_2())
}
fn concat(vec: &[u64]) -> u64 {
    vec.iter().fold(0, |acc, elem| (acc.to_string() + &*elem.to_string()).parse::<u64>().unwrap())
}
fn day6_2() -> u64{
    let p = parse();
    let par = p.iter().map(|line| concat(line)).collect::<Vec<u64>>();
    let possible_vel = get_pos_times(vec![(par[0],par[1])]);
    let mut out = 1;
    for val in possible_vel{
        out *= val;
    }
    out
}

fn get_pos_times(parsed: Vec<(u64,u64)>) -> Vec<u64>{
    parsed.iter().map(|(time,distance)| {
        let mut v_f = 0;
        let mut v_l = *time;

        while v_f*(*time-v_f) <= *distance {
            v_f += 1;
        }
        while v_l*(*time-v_l) <= *distance {
            v_l -= 1;
        }
        v_l-v_f+1
    }).collect::<Vec<u64>>()
}

fn day6_1() -> u64{
    let p = parse();
    let parsed = zip(p[0].clone(),p[1].clone()).collect::<Vec<(u64,u64)>>();
    let possible_vel = get_pos_times(parsed);
    let mut out = 1;
    for val in possible_vel{
        out *= val;
    }
    out

}

fn parse() -> Vec<Vec<u64>> {
    let file_path = "../input";
    if let Ok(contents) = fs::read_to_string(file_path) {
        let split = contents.lines();
        let p = split
            .map(|line|
                line.split(' ')
                    .filter(|&v|!v.is_empty())
                    .collect::<Vec<&str>>()
            )
            .map(|l| l.iter().skip(1).map(|&s|s).collect::<Vec<&str>>())
            .map(|l| l.iter().map(|&num| num.parse::<u64>().unwrap()).collect::<Vec<u64>>())
            .collect::<Vec<Vec<u64>>>();
        return p;
    };
    vec![]
}