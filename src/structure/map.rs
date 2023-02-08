use crate::structure::cell::Cell;
use std::collections::VecDeque;
use std::fs::{self};
pub struct Map {
    pub file_path: String,
    pub start: (usize, usize),
    pub goal: (usize, usize),
    pub search_flag: String,
    pub width: usize,
    pub height: usize,
    pub map: Vec<Vec<Cell>>,
    pub display_map: Vec<Vec<String>>,
    pub solution_cost: usize,
    pub path_dist: usize,
    pub exec_time: u128,
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
        let mut map = Vec::new();
        let mut display_map = Vec::new();
        let mut chars: std::str::Chars;
        for h in 0..height {
            let mut row = Vec::new();
            let mut display_row = Vec::new();
            chars = file_lines[h + 1].chars();
            for w in 0..width {
                let letter = chars.next().expect("Error parsing map characters.");
                row.push(Cell::new(
                    (w, h),
                    None,
                    match letter {
                        'R' => 1,
                        'f' => 2,
                        'F' => 4,
                        'h' => 5,
                        'r' => 7,
                        'M' => 10,
                        _ => usize::MAX,
                    },
                    usize::MAX,
                    usize::MAX,
                ));
                display_row.push(String::from(letter.to_string()));
            }
            map.push(row);
            display_map.push(display_row);
        }
        let solution_cost: usize = 0;
        let exec_time = 0;
        let path_dist = 0;
        Ok(Map {
            file_path,
            start,
            goal,
            search_flag,
            width,
            height,
            map,
            display_map,
            solution_cost,
            path_dist,
            exec_time,
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
        self.display_map[goal_y][goal_x] = String::from("P");
        self.solution_cost = path_cell.weight;
        solution_path.push_back(path_cell.state);
        while path_cell.state != self.start {
            if let Some((x, y)) = path_cell.prev {
                solution_path.push_back((x, y));
                self.display_map[y][x] = String::from("P");
                path_cell = &mut self.map[y][x];
                self.solution_cost += path_cell.weight;
                self.path_dist += 1;
            } else {
                path_cell.prev = None;
            };
        }
        solution_path
    }
    pub fn print_path(&mut self) -> () {
        let mut explored = 0;
        print!(
            "{}",
            self.map
                .iter()
                .map(|row| row
                    .iter()
                    .map(|cell| {
                        if cell.path_cost != usize::MAX
                            && self.display_map[cell.state.1][cell.state.0] != "P"
                        {
                            explored += 1;
                            String::from("~")
                        } else {
                            self.display_map[cell.state.1][cell.state.0].to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        );
        println!("\n------------------------");
        println!("Cells explored (~): {}", explored + self.path_dist);
        println!("Path distance (size): {}", self.path_dist);
        println!("Path (P) cost: {}", self.solution_cost);
        println!("Execution time (nanoseconds): {}", self.exec_time);
    }
}
