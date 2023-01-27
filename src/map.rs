use crate::Config;
use std::error::Error;
use std::fs;

//file contains map and cell structs
#[derive(Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
pub struct Cell {
    pub value: char,
    pub visited: bool,
    pub state: (usize, usize),
    pub prev: Option<(usize, usize)>,
    pub in_path: bool,
    pub prev_direction: Option<Direction>,
}
impl Cell {
    pub fn new(
        value: char,
        visited: bool,
        state: (usize, usize),
        prev: Option<(usize, usize)>,
        in_path: bool,
        prev_direction: Option<Direction>,
    ) -> Cell {
        Cell {
            value,
            visited,
            state,
            prev,
            in_path,
            prev_direction,
        }
    }
    pub fn set_val(&mut self, val: char) -> () {
        self.value = val;
    }
    pub fn set_state(&mut self, coords: (usize, usize)) -> () {
        self.state = coords;
    }
}
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub map: Vec<Vec<Cell>>,
    pub cells_explored: i16,
    pub display_map: Vec<Vec<String>>,
}
impl Map {
    pub fn create(file_path: &String) -> Result<Map, Box<dyn Error>> {
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
        let mut map = vec![vec![Cell::new(' ', false, (0, 0), None, false, None); width]; height];
        let mut chars: std::str::Chars;
        //populate map with correct characters and coordinates for each cell
        for h in 0..height {
            chars = file_lines[h + 1].chars();
            for w in 0..width {
                map[h][w].set_val(chars.next().expect("Error parsing map characters."));
                map[h][w].set_state(((w).try_into().unwrap(), h.try_into().unwrap()));
            }
        }
        let cells_explored: i16 = 0;
        let display_map = vec![vec![String::from(" "); width * 2]; height * 2];
        Ok(Map {
            width,
            height,
            map,
            cells_explored,
            display_map,
        })
    }
    pub fn print_map(&self) -> () {
        println!(
            "{}",
            self.map
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|cell| cell.value.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
    pub fn set_path(&mut self, params: &Config) -> () {
        //identify all cells in the solution path
        let mut path_cell = &mut self.map[params.goal_y][params.goal_x];
        path_cell.in_path = true;
        while path_cell.state != (params.start_x, params.start_y) {
            if let Some((coordx, coordy)) = path_cell.prev {
                self.map[coordy][coordx].in_path = true;
                path_cell = &mut self.map[coordy][coordx];
            } else {
                path_cell.prev = None;
            };
        }
        // set the display map with the path
        for h in 0..self.height {
            for w in 0..self.width {
                self.display_map[2 * h][2 * w] = self.map[h][w].value.to_string();
                if self.map[h][w].in_path {
                    match &self.map[h][w].prev_direction {
                        Some(dir) => match dir {
                            Direction::Up => {
                                self.display_map[(2 * h) - 1][2 * w] = String::from("|")
                            }
                            Direction::Right => {
                                self.display_map[2 * h][(2 * w) + 1] = String::from("_")
                            }
                            Direction::Down => {
                                self.display_map[(2 * h) + 1][2 * w] = String::from("|")
                            }
                            Direction::Left => {
                                self.display_map[2 * h][(2 * w) - 1] = String::from("_")
                            }
                        },
                        None => (),
                    }
                }
            }
        }
    }
    pub fn print_path(&self) -> () {
        println!(
            "{}",
            self.display_map
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|val| val.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect::<Vec<String>>()
                .join("\n")
        );
        println!("------------------------");
        println!("Cells explored: {}", self.cells_explored);
    }
}
