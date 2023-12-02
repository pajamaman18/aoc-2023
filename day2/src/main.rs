use std::fs;

use regex::Regex;

fn main() {
    let am_red = 12;
    let am_blue = 14;
    let am_green = 13;
    // day2_1(am_red,am_green,am_blue)
    println!("day 2 part 1: {}",day2_1(am_red,am_green,am_blue));
    println!("day 2 part 2: {}",day2_2());
    // for (s,g) in parse_file().iter().enumerate(){
    //     println!("{}: {:?} -> {}",s+1,g.clone(),fails(g.clone(),12,13,14));
    // }
}

fn fails(ga: Vec<Vec<u32>>, r:u32, g:u32, b:u32) -> bool {
    !ga.iter()
        .any(|round| round[0]>r || round[1]>g || round[2]>b)
}

fn parse_file() -> Vec<Vec<Vec<u32>>> {
    let file_path = "../input";
    // println!("file path: {}", file_path);
    let re = Regex::new(r"Game [0-9]+: ([0-9a-z,; ]+)").unwrap();
    let mut lines = vec![];
    if let Ok(contents) = fs::read_to_string(file_path) {
        for (_,[balls]) in re.captures_iter(&*contents).map(|c|c.extract()){
            lines.push(parse_balls(balls.to_string()));
        }
    }
    // for thing in lines{
    //     println!("{:?}",thing)
    // }
    lines
}

fn parse_balls(balls: String) -> Vec<Vec<u32>>{
    // split sets (;)
    let split = balls.split("; ");
    let mut game = vec![];
    for set in split{
        let mut red: u32 = 0;
        let mut blue: u32 = 0;
        let mut green: u32 = 0;
        // split pulls (,)
        let set_split = set.split(", ");
            for pull in set_split{
                // split ball and amount ( )
                let amount = pull.split(" ").collect::<Vec<&str>>();
                let am_balls = amount[0].parse::<u32>().unwrap();
                match amount[1] {
                    "red" => red += am_balls,
                    "blue" => blue += am_balls,
                    "green" => green += am_balls,
                    _ => println!("something went wrong with color matching")
                }
            }
        game.push(vec![red, green, blue]);
    }
    game

}

fn day2_1(red:u32,green:u32,blue:u32) -> u32{
    let parsed_data = parse_file();

    let fails= parsed_data
        .iter().enumerate()
        .map(|(game_number,balls_pulled)| {
            if !balls_pulled
                .iter()
                .any(|b| b[0] > red || b[1] > green || b[2] > blue)
            {
                game_number + 1
            }else{
                0
            }
        })
        .filter_map(|gn| {
            if gn as u32 > 0{
                Some(gn as u32)
            }else {
                None
            }
        }).sum();
    fails
    // println!("{:?}",fails)
}

fn day2_2() -> u32{
    let parsed_data = parse_file();
    let a:u32 = parsed_data
        .iter()
        .map(|game|{

            let mut min_r = 0;
            let mut min_g = 0;
            let mut min_b = 0;
            for set in game{
                if set[0] > min_r{
                    min_r = set[0]
                }
                if set[1] > min_g{
                    min_g = set[1]
                }
                if set[2] > min_b{
                    min_b = set[2]
                }
            }
            min_r*min_g*min_b
        }).sum();
    a
}