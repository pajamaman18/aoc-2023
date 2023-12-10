use std::fs;

fn main() {
    // println!("{:?}",day9_1());
    println!("{:?}",day9_2());
}

fn day9_1() -> i64{
    let p = parse();
    let mut out = 0;
    for l in p{
        let mut line = l;
        while line.iter().any(|v|*v != 0){
            out += line.last().unwrap();
            line = calc_next_row(line);

        }
    }
    out
}


fn day9_2() -> i64 {
    let p = parse();
    let mut output_val = 0;
    for mut l in p{
        let mut last_val = vec![];
        l.reverse();
        let mut line = l;
        while line.iter().any(|v|*v != 0){
            last_val.push(*line.last().unwrap());
            line = calc_next_row(line.iter().map(|v| -*v).collect::<Vec<i64>>());
        }
        // println!("{:?}", last_val);
        let mut out = 0;
        for val in last_val.iter().rev(){
            out = val - out;
        }
        output_val += out
    }
    output_val
}
fn calc_next_row(r:Vec<i64>) -> Vec<i64>{
    let mut next_row = vec![];
    for i in 0..r.len()-1{
        next_row.push(r[i+1]-r[i]);
    }
    next_row
}

fn parse() -> Vec<Vec<i64>> {
    let file_path = "../input";
    let mut lines = vec![];
    if let Ok(contents) = fs::read_to_string(file_path) {
        for line in contents.lines(){
            let l = line.split(' ').map(|v| v.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            lines.push(l);
        }
    }
    lines
}