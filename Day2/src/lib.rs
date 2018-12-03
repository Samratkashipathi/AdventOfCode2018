use std::collections::HashMap;
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

        string_list
    }

    pub fn find_multiples(self, string_list: &Vec<String>) -> i32 {
        let mut multiple_has_map = HashMap::new();
        for s in string_list {
            let mut already_inserted_2 = false;
            let mut already_inserted_3 = false;
            let mut temp_hash_map = HashMap::new();
            let char_vec: Vec<char> = s.chars().collect();
            for c in char_vec {
                let count = temp_hash_map.entry(c).or_insert(0);
                *count += 1;
            }
            for (key, value) in temp_hash_map {
                //println!("{}: {}", key, value);
                if value == 2 && !already_inserted_2 {
                    let count = multiple_has_map.entry(value).or_insert(0);
                    *count += 1;
                    already_inserted_2 = true
                }
                if value == 3 && !already_inserted_3 {
                    let count = multiple_has_map.entry(value).or_insert(0);
                    *count += 1;
                    already_inserted_3 = true
                }
            }

            //println!("End of loop")
        }
        let mut multiple = 1;
        for (key, value) in multiple_has_map {
            multiple *= value
        }

        multiple
    }
}

