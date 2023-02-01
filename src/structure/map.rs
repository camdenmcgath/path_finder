use crate::structure::{cell::Cell, direction::Direction};
use crate::Config;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;

//file contains map and cell structs
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub map: Vec<Vec<Cell>>,
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
        let mut map =
            vec![vec![Cell::new(' ', (0, 0), None, false, None, usize::MAX); width]; height];
        let mut chars: std::str::Chars;
        //populate map with correct characters and coordinates for each cell
        for h in 0..height {
            chars = file_lines[h + 1].chars();
            for w in 0..width {
                map[h][w].set_val(chars.next().expect("Error parsing map characters."));
                map[h][w].set_state(((w).try_into().unwrap(), h.try_into().unwrap()));
            }
        }
        let display_map = vec![vec![String::from(" "); width * 2]; height * 2];
        Ok(Map {
            width,
            height,
            map,
            display_map,
        })
    }
    pub fn get(&mut self, point: &(usize, usize)) -> &mut Cell {
        &mut self.map[point.1][point.0]
    }
    pub fn find_prev_direction(
        &self,
        origin: &(usize, usize),
        point: &(usize, usize),
    ) -> Option<Direction> {
        let (origin_x, origin_y) = *origin;
        let (point_x, point_y) = *point;

        //moved up
        if origin_y > 0 && point_y == origin_y - 1 {
            Some(Direction::Down)
        }
        //moved right
        else if origin_x + 1 < self.width && point_x == origin_x + 1 {
            Some(Direction::Left)
        }
        //moved down
        else if origin_y + 1 < self.height && point_y == origin_y + 1 {
            Some(Direction::Up)
        }
        //moved left
        else if origin_x > 0 && point_x == origin_x - 1 {
            Some(Direction::Right)
        } else {
            None
        }
    }
    pub fn set_cell_costs(&mut self) -> () {
        self.map.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                cell.cost = match cell.value {
                    'R' => 1,
                    'f' => 2,
                    'F' => 4,
                    'h' => 5,
                    'r' => 7,
                    'M' => 10,
                    _ => usize::MAX,
                };
            });
        });
    }
    pub fn expand(&mut self, point: &(usize, usize)) -> VecDeque<(usize, usize)> {
        let (state_x, state_y) = *point;
        let mut expanded_cells: VecDeque<(usize, usize)> = VecDeque::new();
        //move up
        if state_y > 0 && self.map[state_y - 1][state_x].value != 'W' {
            expanded_cells.push_back(self.map[state_y - 1][state_x].state);
        }
        //move right
        if state_x + 1 < self.width && self.map[state_y][state_x + 1].value != 'W' {
            expanded_cells.push_back(self.map[state_y][state_x + 1].state);
        }
        //move down
        if state_y + 1 < self.height && self.map[state_y + 1][state_x].value != 'W' {
            expanded_cells.push_back(self.map[state_y + 1][state_x].state);
        }
        //move left
        if state_x > 0 && self.map[state_y][state_x - 1].value != 'W' {
            expanded_cells.push_back(self.map[state_y][state_x - 1].state);
        }
        expanded_cells
    }
    pub fn set_path(&mut self, point: (usize, usize), params: &Config) -> () {
        //identify all cells in the solution path
        let (goal_x, goal_y) = point;
        let mut path_cell = &mut self.map[goal_y][goal_x];
        path_cell.in_path = true;
        while path_cell.state != params.start {
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
        println!("Cells explored: {}", self.get_explored_cells());
    }
    pub fn get_explored_cells(&self) -> usize {
        self.map
            .iter()
            .flatten()
            .filter(|cell| cell.cost != usize::MAX)
            .count()
    }
}
