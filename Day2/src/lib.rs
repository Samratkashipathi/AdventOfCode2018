extern crate itertools;

use std::collections::HashMap;
use itertools::Itertools;
use std::fs;

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename: filename })
    }

    pub fn read_file(&self) -> Vec<String> {
        let content = fs::read_to_string(&self.filename).unwrap_or_else(|err| {
            panic!("Could not read the file/File path is wrong");
        });

        let mut string_list: Vec<String> = vec![];
        for s in content.split('\n') {
            string_list.push(s.trim().to_string());
        }

        return string_list;
    }

    pub fn find_multiples(&self, string_list: &Vec<String>) -> i32 {
        let mut count_2 = 0;
        let mut count_3 = 0;
        for s in string_list {
            let mut temp_hash_map = HashMap::new();
            let char_vec: Vec<char> = s.chars().collect();
            for c in char_vec {
                let count = temp_hash_map.entry(c).or_insert(0);
                *count += 1;
            }

            if temp_hash_map.values().any(|&c| c == 2) {
                count_2 += 1;
            }

            if temp_hash_map.values().any(|&c| c == 3) {
                count_3 += 1;
            }
        }
        return count_2 * count_3;
    }

    pub fn find_nearest_strings(&self, string_list: &Vec<String>) -> Option<String> {
        let mut nearest_strings: Option<String> = None;
        for (x, y) in string_list.iter().tuple_combinations() {
            let dis = x.chars().zip(y.chars()).filter(|&(a, b)| a != b).count();
            if dis <= 1 {
                nearest_strings = Some(x.chars()
                    .zip(y.chars())
                    .filter_map(|(a, b)|
                        if a == b {
                            Some(a)
                        } else { None }).collect());
                break;
            }
        }
        return nearest_strings;
    }
}

