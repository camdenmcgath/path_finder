use searches::{
    breadth_first::breadth_first, dijkstra::dijkstra, iterative_deepening::iterative_deepening,
};
use std::error::Error;
use structure::map::Map;
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
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let start = match (args.next(), args.next()) {
            (Some(arg), Some(arg2)) => (
                arg.parse::<usize>().unwrap(),
                arg2.parse::<usize>().unwrap(),
            ),
            _ => return Err("Didn't get start coordinates"),
        };
        let goal = match (args.next(), args.next()) {
            (Some(arg), Some(arg2)) => (
                arg.parse::<usize>().unwrap(),
                arg2.parse::<usize>().unwrap(),
            ),
            _ => return Err("Didn't get goal coordinates"),
        };
        let search_flag = match args.next() {
            Some(arg) => arg,
            None => return Err("Search flag not found"),
        };

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
            if let Some(point) = breadth_first(&config, &mut map) {
                map.set_path(point, &config);
                map.print_path();
            } else {
                println!("Goal not found...");
            }
        }
        "-l" => {
            if let Some(point) = dijkstra(&config, &mut map) {
                map.set_path(point, &config);
                map.print_path();
            } else {
                println!("Oops");
            }
        }
        "-i" => {
            if let Some(point) = iterative_deepening(&config, &mut map) {
                map.set_path(point, &config);
                map.print_path();
            } else {
                println!("Oops");
            }
        }
        "-a1" => println!("A* heurstic version 1 not yet implemented"),
        "-a2" => println!("A* huritic version 2 not yet implemented"),
        _ => println!("Please pass one of -b, -l, -i, -a1, -a2 to specify search"),
    }

    Ok(())
}
