use std::fs;

fn main() {
    // println!("day 3 part 1: {}", day4_1());
    println!("day 3 part 2: {}", day4_2());
}

fn day4_1() -> u32 {
    let input = parse_input();
    // for i in input{
    //     println!("{:?}",i);
    // }
        let mut points = 0;
    for (win_num,actual_num) in input{
        let wins = actual_num.iter().filter_map(|num|{
            if win_num.contains(num){
                Some(1)
            }else {
                None
            }
        }).sum::<u32>();
        if wins >= 1{
            points += 2u32.pow(wins-1);

        }
    }
    points
}

fn day4_2() -> u32{
    let input = parse_input();
    let mut amount_of_cards: [u32; 206] = [0;206];
    amount_of_cards = amount_of_cards.map(|v| 1);
    for (card_no, (win_num,actual_num)) in input.iter().enumerate(){
        let wins = actual_num.iter().filter_map(|num|{
            if win_num.contains(num){
                Some(1)
            }else {
                None
            }
        }).sum::<usize>();
        for index in card_no+1..card_no+wins+1{
            amount_of_cards[index] += 1*amount_of_cards[card_no]
        }

    }
    amount_of_cards.iter().sum()
}

fn parse_input() -> Vec<(Vec<u32>,Vec<u32>)> {
    let file_path = "/home/pjman/personal-projects/aoc-2023-rust/input";
    if let Ok(contents) = fs::read_to_string(file_path) {
        let split_lines = contents.lines();
        return split_lines.map(|line|line.split(": ").nth(1).unwrap())
            .map(|nums| {
                // println!("{}",nums);
                let mut split = nums.split(" | ");
                let wins = split.clone().nth(0).unwrap();
                let numbers = split.nth(1).unwrap();
                let uwins = wins.split(" ").filter(|a|!a.is_empty()).map(|val|val.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                let unumbers = numbers.split(" ").filter(|a|!a.is_empty()).map(|val|val.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                (uwins,unumbers)
            })
            .collect::<Vec<(Vec<u32>,Vec<u32>)>>();
    };
    vec![]

}