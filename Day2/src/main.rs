extern crate Day2;

use std::env;
use std::process;

use Day2::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem while parsing the arguments");
        process::exit(1);
    });

    let str_list = config.read_file();

    //println!("{:#?}", str_list);

    println!("{}", config.find_multiples(&str_list));
    println!("{:#?}", config.find_nearest_strings(&str_list));
}
