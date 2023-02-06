use crate::structure::cell::Cell;
use std::collections::VecDeque;
use std::fs;

//file contains map and cell structs
pub struct Map {
    pub file_path: String,
    pub start: (usize, usize),
    pub goal: (usize, usize),
    pub search_flag: String,
    pub width: usize,
    pub height: usize,
    pub map: Vec<Vec<Cell>>,
    pub display_map: Vec<Vec<String>>,
}
impl Map {
    pub fn create(mut args: impl Iterator<Item = String>) -> Result<Map, &'static str> {
        //parse command line args
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

        let contents = fs::read_to_string(&file_path).expect("Problem reading file to string");
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
        //initialize map
        let mut map =
            vec![vec![Cell::new((0, 0), None, usize::MAX, usize::MAX, usize::MAX); width]; height];
        //initialize map for display
        let mut display_map = vec![vec![String::from(" "); width * 2]; height * 2];
        let mut chars: std::str::Chars;
        //set map values based on input grid
        for h in 0..height {
            chars = file_lines[h + 1].chars();
            for w in 0..width {
                let letter = chars.next().expect("Error parsing map characters.");
                display_map[h * 2][w * 2] = letter.to_string();
                map[h][w].state = ((w).try_into().unwrap(), h.try_into().unwrap());
                map[h][w].weight = match letter {
                    'R' => 1,
                    'f' => 2,
                    'F' => 4,
                    'h' => 5,
                    'r' => 7,
                    'M' => 10,
                    _ => usize::MAX,
                };
            }
        }

        Ok(Map {
            file_path,
            start,
            goal,
            search_flag,
            width,
            height,
            map,
            display_map,
        })
    }
    pub fn get(&mut self, point: &(usize, usize)) -> &mut Cell {
        &mut self.map[point.1][point.0]
    }
    pub fn expand(&mut self, point: &(usize, usize)) -> VecDeque<(usize, usize)> {
        let (state_x, state_y) = *point;
        let mut expanded_cells: VecDeque<(usize, usize)> = VecDeque::new();
        //move up
        if state_y > 0 && self.map[state_y - 1][state_x].weight != usize::MAX {
            expanded_cells.push_back(self.map[state_y - 1][state_x].state);
        }
        //move right
        if state_x + 1 < self.width && self.map[state_y][state_x + 1].weight != usize::MAX {
            expanded_cells.push_back(self.map[state_y][state_x + 1].state);
        }
        //move down
        if state_y + 1 < self.height && self.map[state_y + 1][state_x].weight != usize::MAX {
            expanded_cells.push_back(self.map[state_y + 1][state_x].state);
        }
        //move left
        if state_x > 0 && self.map[state_y][state_x - 1].weight != usize::MAX {
            expanded_cells.push_back(self.map[state_y][state_x - 1].state);
        }
        expanded_cells
    }
    pub fn set_path(&mut self, point: (usize, usize)) -> VecDeque<(usize, usize)> {
        //identify all cells in the solution path
        let mut solution_path = VecDeque::<(usize, usize)>::new();
        let (goal_x, goal_y) = point;
        let mut path_cell = &mut self.map[goal_y][goal_x];
        //path_cell.in_path = true;
        solution_path.push_back(path_cell.state);
        while path_cell.state != self.start {
            if let Some((x, y)) = path_cell.prev {
                solution_path.push_back((x, y));
                let (origin_x, origin_y) = (x, y);
                let (point_x, point_y) = path_cell.state;
                //moved up
                if origin_y > 0 && point_y == origin_y - 1 {
                    self.display_map[(2 * y) - 1][2 * x] = String::from("|");
                }
                //moved right
                else if origin_x + 1 < self.width && point_x == origin_x + 1 {
                    self.display_map[2 * y][(2 * x) + 1] = String::from("_");
                }
                //moved down
                else if origin_y + 1 < self.height && point_y == origin_y + 1 {
                    self.display_map[(2 * y) + 1][2 * x] = String::from("|");
                }
                //moved left
                else if origin_x > 0 && point_x == origin_x - 1 {
                    self.display_map[2 * y][(2 * x) - 1] = String::from("_");
                }
                path_cell = &mut self.map[y][x];
            } else {
                path_cell.prev = None;
            };
        }
        solution_path
    }
    pub fn print_path(&mut self) -> () {
        //self.set_display(path);
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
        println!("Path cost: {}", self.get_solution_cost());
    }
    pub fn get_explored_cells(&self) -> usize {
        self.map
            .iter()
            .flatten()
            .filter(|cell| cell.path_cost != usize::MAX)
            .count()
    }
    pub fn get_solution_cost(&self) -> usize {
        let mut solution_cost: usize = 0;
        let mut cell = &self.map[self.goal.1][self.goal.0];
        while cell.state != self.start {
            solution_cost += cell.weight;
            if let Some((x, y)) = cell.prev {
                cell = &self.map[y][x];
            } else {
            }
        }
        solution_cost
    }
}
