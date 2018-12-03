use std::env;
use std::process;
use std::error::Error;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Filename not passed");
    }

    let filename = args[1].clone();

    let contents = read_file(filename);

    let freq_list = get_freq_list(contents);

    println!("{:#?}", calculate_freq_sum(&freq_list));
    println!("{:#?}", first_repeated_freq(&freq_list));
}

fn read_file(filename: String) -> String {
    let content = fs::read_to_string(filename).unwrap_or_else(|err| {
        panic!("Could not read the file/File path is wrong");
    });

    content
}

fn get_freq_list(content: String) -> Vec<i64> {
    let mut fr: Vec<i64> = vec![];

    for s in content.split('\n') {
        match s.trim().parse::<i64>() {
            Ok(n) => fr.push(n),
            Err(e) => panic!("{}", e)
        }
    }

    fr
}

fn calculate_freq_sum(freq: &Vec<i64>) -> i64 {
    let mut sum: i64 = 0;
    for i in freq.iter() {
        sum += i;
    }

    sum
}

fn first_repeated_freq(freq: &Vec<i64>) -> i64 {
    let mut found_freq = HashSet::new();
    let mut repeated = 0;
    let mut is_repeated = false;
    let mut sum = 0;

    while !is_repeated {
        for i in freq.iter() {
            sum += i;

            if !is_repeated && found_freq.contains(&sum) {
                is_repeated = true;
                repeated = sum;
                break;
            }

            found_freq.insert(sum);
        }
    }
    repeated
}


