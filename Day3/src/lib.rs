extern crate regex;

use std::fs;
use regex::Regex;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Rectangle {
    pub claim_id: i64,
    pub left_edge: i64,
    pub top_edge: i64,
    pub width: i64,
    pub height: i64,
}

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

    pub fn read_file(&self) -> Vec<Rectangle> {
        let content = fs::read_to_string(&self.filename).unwrap_or_else(|err| {
            panic!("Could not read the file/File path is wrong");
        });

        let mut data: Vec<Rectangle> = vec![];
        for s in content.split('\n') {
            //println!("{}", s);
            data.push(match Config::parse_input(s.trim().to_string()) {
                Some(x) => x,
                None => panic!("parsing input failed")
            });
        }

        return data;
    }

    fn parse_input(data: String) -> Option<Rectangle> {
        //println!("Parsing Input");
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap_or_else(|err| {
            panic!("Could not create new regex")
        });

        for cap in re.captures(&data) {
            //println!("{:#?}", cap);
            let rect = Rectangle {
                claim_id: cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                left_edge: cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                top_edge: cap.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                width: cap.get(4).unwrap().as_str().parse::<i64>().unwrap(),
                height: cap.get(5).unwrap().as_str().parse::<i64>().unwrap(),
            };

            return Some(rect);
        }

        return None;
    }
}