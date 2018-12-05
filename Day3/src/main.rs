extern crate Day3;

use std::env;
use std::process;
use std::collections::HashMap;

use Day3::Config;
use Day3::Rectangle;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem while parsing the arguments");
        process::exit(1);
    });

    let data = config.read_file();

    // println!("{:#?}", data);

    find_overlap(&data);
}


fn find_overlap(data: &Vec<Rectangle>) {
    let mut grid = HashMap::new();

    for rect in data {
        for row in rect.top_edge + 1..rect.height + rect.top_edge + 1 {
            for col in rect.left_edge + 1..rect.width + rect.left_edge + 1 {
                let count = grid.entry((row, col)).or_insert(0);
                *count += 1;
            }
        }
    }

    let mut overlap_count = grid.values().filter(|x| x >= &&2).count();
    println!("Overlap = {}", overlap_count);
}
