use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
    pub start_x: String,
    pub start_y: String,
    pub goal_x: String,
    pub goal_y: String,
    pub search_flag: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 6 {
            return Err("Not enough arguments!");
        }
        let file_path = args[1].clone();
        let start_x = args[2].clone();
        let start_y = args[3].clone();
        let goal_x = args[4].clone();
        let goal_y = args[5].clone();
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
    let _map = fs::read_to_string(config.file_path)?;
    Ok(())
}
