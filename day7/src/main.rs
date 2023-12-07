use std::fs;

fn main() {
    // let mut p:Vec<Vec<usize>> = vec![];
    // p.push(vec![1,2,3]);
    // p.push(vec![2,3,4]);
    // p.push(vec![2,3,5]);
    // p.push(vec![2,3,3]);
    // p.sort();
    // print!("{:?}",p);
    // println!("day 7 part 1: {}",day7_1());
    println!("day 7 part 1: {}", day7_2());
    // let a = vec![1usize,2,3,100];
    // let b = vec![1,2,4,1];
    // println!("{}",a<b)
}


fn day7_1() -> usize {
    let p = parse(true);
    let mut sublists: Vec<Vec<(Vec<usize>, usize)>> = vec![];
    for _ in 0..7 {
        sublists.push(vec![]);
    }
    for (score, hand, bid) in p {
        sublists[score - 1].push((hand, bid));
    }
    // println!("{:?}",sublists);
    let non_empty_sublists = sort_sublist(sublists);

    // println!("{:?}",non_empty_sublists);

    non_empty_sublists.iter().flatten().enumerate().map(|(index, bid)| (index + 1) * *bid).sum()
}

fn day7_2() -> usize {
    let p = parse(false);
    let mut sublists: Vec<Vec<(Vec<usize>, usize)>> = vec![];
    for _ in 0..7 {
        sublists.push(vec![]);
    }
    for (score, hand, bid) in p {
        sublists[score - 1].push((hand, bid));
    }
    println!("{:?}", sublists);
    let non_empty_sublists = sort_sublist(sublists);

    println!("{:?}", non_empty_sublists);

    non_empty_sublists.iter().flatten().enumerate().map(|(index, bid)| (index + 1) * *bid).sum()
}

fn sort_sublist(sublists: Vec<Vec<(Vec<usize>, usize)>>) -> Vec<Vec<usize>> {
    sublists.iter().filter_map(|hands| {
        if hands.len() > 0 {
            Some({
                let mut mapped = hands.iter().map(|(hand, bid)| {
                    let mut appended_hand = hand.clone();
                    appended_hand.push(*bid);
                    appended_hand
                })
                    .collect::<Vec<Vec<usize>>>();
                // println!("{:?}",mapped);

                mapped.sort();
                mapped.iter().map(|mut list| *list.last().unwrap()).collect::<Vec<usize>>()
            })
        } else {
            None
        }
    })
        .collect::<Vec<Vec<usize>>>()
}


fn parse(is_part1: bool) -> Vec<(usize, Vec<usize>, usize)> {
    let file_path = "../input";
    if let Ok(contents) = fs::read_to_string(file_path) {
        let split = contents.lines();
        return split
            .map(|line| {
                let sp = line.split(' ').collect::<Vec<&str>>();
                (sp[0].to_string(), sp[1].to_string())
            })
            .map(|(hand, bid)| {
                (translate_hand(hand.clone(), is_part1), translate_card_values(hand, is_part1), bid.parse::<usize>().unwrap())
            })
            .collect::<Vec<(usize, Vec<usize>, usize)>>();
    }
    vec![]
}

fn translate_hand(string_hand: String, is_part1: bool) -> usize {
    let hand = translate_card_values(string_hand, is_part1);
    let mut cards: [usize; 14] = [0; 14];
    for card in hand {
        cards[card] += 1;
    }
    // let cards = cards.iter().filter(|&&c|c>0).collect::<Vec<u32>>();
    let max_val = *cards.iter().max().unwrap();
    if is_part1 || cards[0] == 0 {
        match max_val {
            5 => 7,//5oak
            4 => 6,//4oak
            1 => 1,//high card
            _ => {
                let length = cards.iter().filter(|&&c| c > 1).collect::<Vec<&usize>>().len();
                match max_val {
                    3 => max_val + 1 + length - 1,//3oak or flush
                    2 => max_val + length - 1,//pair or two pairs
                    _ => 0
                }
            }
        }
    } else {
        let j_val = cards[0];
        let diff_num = cards.iter().filter(|&&v| v > 0).collect::<Vec<&usize>>().len();
        if max_val == 4 { // 5oak (1J+4X||4J+1X)
            7
        } else if max_val == 3 {
            if diff_num == 3 { //(1J+(1X+3Y)||3J+(1X+1Y)
                6 // 4oak
            } else {//(2J+3X||3J+2X)
                7
            }
        } else if max_val == 1 {//high card
            2
        } else if max_val == 2 {//2+1+1+1|2+2+1

            if diff_num == 3 {
                if j_val == 2 {
                    6//4oak
                } else {
                    5//flush
                }
            } else {
                4
            }
        } else {
            7
        }
    }
}

fn translate_card_values(hand: String, is_part1: bool) -> Vec<usize> {
    hand.chars().map(|c| map_val(c, is_part1)).collect::<Vec<usize>>()
}

fn map_val(c: char, is_part1: bool) -> usize {
    if let Some(val) = c.to_digit(10) {
        (val - 1) as usize
    } else {
        let v = match c {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => {
                if is_part1 {
                    10
                } else {
                    0
                }
            }
            'T' => 9,
            _ => 0
        };
        v as usize
    }
}