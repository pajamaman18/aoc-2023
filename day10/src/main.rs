use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::stdout;

use priority_queue::PriorityQueue;

fn main() {
    println!("day 10 part1: {}",day10_1());
    println!("day 10 part2: {}",day10_2());


}

fn day10_2() -> usize{
    let (spots, map) = parse();
    let mut in_line = HashSet::new();
    let mut is_in = false;
    let mut up = false;
    for (y,row) in map.iter().cloned().enumerate(){
        for (x,c) in row.iter().cloned().enumerate(){
            if !spots.contains_key(&(y,x)){
                if is_in{
                    in_line.insert((y,x));
                }
            }else{
                if c == '|'{
                    is_in = !is_in;
                }else if c == 'L' {
                    up = true;
                }else if c == 'F' {
                    up = false;
                }else if c == '7' {
                    if up{
                        is_in = !is_in;
                    }
                }else if c == 'J' {
                    if !up{
                        is_in = !is_in;
                    }
                }else {
                    if is_in{
                        in_line.insert((y,x));
                    }
                }
            }
        }
    }
    // let not_within = (spots.keys().cloned().collect::<HashSet<(usize,usize)>>()).intersection(&in_line).cloned().collect::<HashSet<(usize,usize)>>();
    in_line.iter().count()
}

fn get_not_within(map: &Vec<Vec<char>>) -> HashSet<(usize, usize)>{
    let mut out = HashSet::new();
    let mut within = false;
    let mut up = false;
    for (y,row) in map.iter().cloned().enumerate(){
        for (x,c) in row.iter().cloned().enumerate(){
            if c == '|'{
                within = !within;
            }else if "LF".contains(c){
                up = c == 'L';
            }else if "J7".contains(c){
                let other = if up{
                    'J'
                }else{
                    'L'
                };
                if c != other{
                    within = !within;
                }
                up = false;
            }
            if !within{
                out.insert((y,x));
            }
        }
    }
    out
}

fn get_s_val(y:usize, x:usize, map: &Vec<Vec<char>>) -> char{
    let up = map[y-1][x];
    let down = map[y+1][x];
    let left = map[y][x-1];
    let right = map[y][x+1];
    if "|7F".contains(up){
        if "J7-".contains(right){
            'L'
        }else if "FL-".contains(left){
            'J'
        }else{
            '|'
        }
    }else if "|JL".contains(down){
        if "j7F".contains(right){
            '7'
        }else{
            'F'
        }
    }else{
        '-'
    }
}

fn day10_1() -> u32{
    let (spots,_) = parse();
    *spots.values().max().unwrap()
}

fn get_surrounding_pipes(y:usize, x:usize, map: &Vec<Vec<char>>) -> Vec<(usize, usize)>{
    let length = 140;
    let mut spots_to_check = vec![];
    if x + 1<= length-1 && !".|LF".contains(map[y][x+1]){ // cell to the right
        spots_to_check.push((y,x+1));
    }
    if x > 0 && !".|J7".contains(map[y][x-1]){ // cell to the left
        spots_to_check.push((y,x-1));
    }
    if y + 1<= length-1 && !".-7F".contains(map[y+1][x]){ // cell below
        spots_to_check.push((y+1,x));
    }
    if y > 0 && !".-LJ".contains(map[y-1][x]){ // cell above
        spots_to_check.push((y-1,x));
    }
    spots_to_check
}
fn parse() -> (HashMap<(usize,usize),u32>,Vec<Vec<char>>) {
    let file_path = "../input";
    let length = 141;
    if let Ok(contents) = fs::read_to_string(file_path) {
        let thing = contents.find("S").unwrap();
        let mut spots_to_check = PriorityQueue::new();
        let x = thing%length;
        let y = thing/length;

        let mut map = contents.lines().map(|a|a.to_string().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let pos_spots = get_surrounding_pipes(y,x,&map);
        for s in pos_spots{
            spots_to_check.push(s,Reverse(1));
        }

        let mut walked:HashMap<(usize,usize),u32> = HashMap::new();
        while !spots_to_check.is_empty() {
            let (pipe,prio) = spots_to_check.pop().unwrap();
            if !walked.contains_key(&pipe){
                walked.insert(pipe,prio.0);
                let new = get_surrounding_pipes(pipe.0,pipe.1,&map);
                for n in new{
                    spots_to_check.push(n,Reverse(prio.0+1));
                }
            }
        }
        map[y][x] = get_s_val(y,x,&map);

        return (walked,map);
        // for line in contents.lines(){
        //     let l = line.split(' ').map(|v| v.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        //     lines.push(l);
        // }
    }
    (HashMap::new(),vec![])
}