use searches::breadth_first::breadth_first_search;
use std::error::Error;
use structure::map::Map;
mod searches;
mod structure;
//struct for Command Line Arguments
pub struct Config {
    pub file_path: String,
    pub start_x: usize,
    pub start_y: usize,
    pub goal_x: usize,
    pub goal_y: usize,
    pub search_flag: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 6 {
            return Err("Not enough arguments!");
        } else if args[6] != "-b" {
            return Err("Please pass the -b flag for breadth first search. Breadth first search is currently the only search implemented.");
        }
        let file_path = args[1].clone();
        let start_x = args[2].clone().parse::<usize>().unwrap();
        let start_y = args[3].clone().parse::<usize>().unwrap();
        let goal_x = args[4].clone().parse::<usize>().unwrap();
        let goal_y = args[5].clone().parse::<usize>().unwrap();
        let search_flag = args[6].clone();

        Ok(Config {
            file_path,
            start_x,
            start_y,
            goal_x,
            goal_y,
            search_flag,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{}", std::env::current_dir().unwrap().display());
    let mut map = Map::create(&config.file_path)?;
    if let Some(cell) = breadth_first_search(&config, &mut map) {
        map.set_path(&cell, &config);
        map.print_path();
    } else {
        println!("Goal not found...");
    }

    Ok(())
}
