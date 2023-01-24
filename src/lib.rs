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
#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    map: Vec<Vec<char>>,
}

impl Map {
    fn create(file_path: &String) -> Result<Map, Box<dyn Error>> {
        let contents = fs::read_to_string(file_path)?;
        let file_lines = contents.lines().collect::<Vec<&str>>();
        let mut dimensions = file_lines[0].split(' ');
        let width = dimensions
            .next()
            .expect("Error parsing dimensions from file.")
            .parse::<usize>()
            .unwrap();
        let height = dimensions
            .next()
            .expect("Error parsing dimensions from file.")
            .parse::<usize>()
            .unwrap();
        let mut map = vec![vec![' '; width]; height];
        let mut chars: std::str::Chars;
        for h in 1..height {
            chars = file_lines[h].chars();
            for w in 0..width {
                map[h][w] = chars.next().expect("Error parsing map characters.");
            }
        }
        Ok(Map { width, height, map })
    }
    fn size(&self) -> usize {
        self.width * self.height
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{}", std::env::current_dir().unwrap().display());

    let map = Map::create(&config.file_path)?;
    //dbg!(&map);
    for row in 1..map.height() {
        for column in 0..map.width() {
            //add print of map
        }
    }
    Ok(())
}
