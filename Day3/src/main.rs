extern crate Day3;

use std::env;
use std::process;

use Day3::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem while parsing the arguments");
        process::exit(1);
    });

    let data = config.read_file();


    println!("{:#?}", data);


}
