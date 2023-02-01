use searches::breadth_first::breadth_first_search;
use std::error::Error;
use structure::map::Map;

use crate::searches::djikstra::djikstra_search;
mod searches;
mod structure;

//struct for Command Line Arguments
pub struct Config {
    pub file_path: String,
    pub start: (usize, usize),
    pub goal: (usize, usize),
    pub search_flag: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 6 {
            return Err("Not enough arguments!");
        }
        let file_path = args[1].clone();
        let start = (
            args[2].clone().parse::<usize>().unwrap(),
            args[3].clone().parse::<usize>().unwrap(),
        );
        let goal = (
            args[4].clone().parse::<usize>().unwrap(),
            args[5].clone().parse::<usize>().unwrap(),
        );
        let search_flag = args[6].clone();

        Ok(Config {
            file_path,
            start,
            goal,
            search_flag,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{}", std::env::current_dir().unwrap().display());
    let mut map = Map::create(&config.file_path)?;
    match config.search_flag.as_str() {
        "-b" => {
            if let Some(point) = breadth_first_search(&config, &mut map) {
                map.set_path(point, &config);
                map.print_path();
            } else {
                println!("Goal not found...");
            }
        }
        "-l" => {
            if let Some(point) = djikstra_search(&config, &mut map) {
                map.set_path(point, &config);
                map.print_path();
            } else {
                println!("Oops");
            }
        }
        "-i" => println!("Iterative deepening not yet implemented"),
        "-a1" => println!("A* heurstic version 1 not yet implemented"),
        "-a2" => println!("A* huritic version 2 not yet implemented"),
        _ => println!("Please pass one of -b, -l, -i, -a1, -a2 to specify search"),
    }

    Ok(())
}
