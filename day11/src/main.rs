use std::fs;

fn main() {
    // for l in parse(){
    //     println!("{:?}",l)
    // }
    println!("day 11 part1: {}", day11_1());
    println!("day 11 part2: {}", day11_2());
}

fn day11_2() -> u64 {
    let (map, mut galaxy_coords) = parse();
    solve_expansion(map, &mut galaxy_coords, 1000000usize);
    calc_dist(galaxy_coords)
}

fn day11_1() -> u64 {
    let (map, mut galaxy_coords) = parse();
    solve_expansion(map, &mut galaxy_coords, 2usize);
    calc_dist(galaxy_coords)
}

fn calc_dist(galaxy_coords: Vec<(usize, usize)>) -> u64 {
    let mut distances: u64 = 0;
    let length = galaxy_coords.len();
    for galaxy in 0..length {
        let (origin_galaxy_y, origin_galaxy_x) = galaxy_coords[galaxy];

        for other_galaxy in galaxy..length {
            let (other_galaxy_y, other_galaxy_x) = galaxy_coords[other_galaxy];
            distances += ((other_galaxy_x as i32 - origin_galaxy_x as i32).abs() + (other_galaxy_y as i32 - origin_galaxy_y as i32).abs()) as u64
        }
    }
    distances
}

fn solve_expansion(map: Vec<Vec<u64>>, coords: &mut Vec<(usize, usize)>, expansion: usize) {
    let actual_expansion = expansion - 1;
    let mut rows = vec![];
    for (j, row) in map.iter().enumerate() {
        if row.iter().sum::<u64>() == 0 {
            rows.push(j);
        }
    }
    update_row_coords(coords, rows, actual_expansion);
    let mut cols = vec![];
    for i in 0..map[0].len() {
        let mut col_val = 0;
        for j in 0..map.len() {
            col_val += map[j][i];
        }
        if col_val == 0 {
            cols.push(i);
        }
    }
    update_col_coords(coords, cols, actual_expansion);
}

fn update_row_coords(coords: &mut Vec<(usize, usize)>, rows: Vec<usize>, expansion: usize) {
    let mut to_update = vec![];
    for exp_row in rows {
        for (row, i) in coords.iter().enumerate() {
            if i.0 > exp_row {
                to_update.push(row);
            }
        }
    }
    for needs_updating in to_update {
        coords[needs_updating] = (coords[needs_updating].0 + expansion, coords[needs_updating].1);
    }
}

fn update_col_coords(coords: &mut Vec<(usize, usize)>, cols: Vec<usize>, expansion: usize) {
    let mut to_update = vec![];
    for exp_col in cols {
        for (col, i) in coords.iter().enumerate() {
            if i.1 > exp_col {
                to_update.push(col);
            }
        }
    }
    for needs_updating in to_update {
        coords[needs_updating] = (coords[needs_updating].0, coords[needs_updating].1 + expansion);
    }
}


fn parse() -> (Vec<Vec<u64>>, Vec<(usize, usize)>) {
    let file_path = "../input";
    let mut out = vec![];
    let mut coords = vec![];
    if let Ok(contents) = fs::read_to_string(file_path) {
        for (j, line) in contents.lines().enumerate() {
            out.push(line.chars().enumerate().map(|(i, c)| {
                if c == '.' {
                    0
                } else {
                    coords.push((j, i));
                    1
                }
            }).collect::<Vec<u64>>()
            )
        }
    }


    (out, coords)
}