use std::collections::VecDeque;
use std::error::Error;
use std::fs;

/*

//BFS- no cost, depth only
//Lowest cost- Djik. PQ path cost for comparing
//Iterative Deep by cost -
//A*- PQ fn with own mechanisms which make it faster
Comparator(i1, i2)
    //unifrom cost search
    if (i1.cost > i2.cost) {
        return 1
    }
    else {
        return -1
    }
    //breadth first search (switch >/< for DFS or BFS)
    if (i1.depth > i2.depth) {
        return 1
    }
    else {
        return -1
    }
MyPQ = PriorityQue(myComparator)
MyPQ.Add(mynewnode)
*/

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
//#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    map: Vec<Vec<Cell>>,
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
        let mut map = vec![vec![Cell::new(' ', false, (0, 0), None, false); width]; height];
        let mut chars: std::str::Chars;
        for h in 0..height {
            chars = file_lines[h + 1].chars();
            for w in 0..width {
                map[h][w].set_val(chars.next().expect("Error parsing map characters."));
                map[h][w].set_state(((w).try_into().unwrap(), h.try_into().unwrap()));
            }
        }
        Ok(Map { width, height, map })
    }
    fn print(&self) -> () {
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
    fn set_path(&mut self, cell: Cell) -> () {
        let (x, y) = cell.state;
        let mut path_cell = &mut self.map[x][y];
        path_cell.in_path = true;
        while path_cell.prev != None {
            dbg!(&path_cell);
            if let Some((coord1, coord2)) = path_cell.prev {
                self.map[coord1][coord2].in_path = true;
                path_cell = &mut self.map[coord1][coord2];
            } else {
                path_cell.prev = None;
            };
        }
        println!("Exited loop!");
    }
    fn print_path(&self) -> () {
        println!(
            "{}",
            self.map
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|cell| {
                            if cell.in_path {
                                cell.value.to_string()
                            } else {
                                String::from(" ")
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[derive(Clone, Debug)]
struct Cell {
    value: char,
    //cost: Option<i32>,
    //prev: Option<(i32, i32)>,
    visited: bool,
    //action: Option<String>,
    state: (usize, usize),
    prev: Option<(usize, usize)>,
    in_path: bool,
}
impl Cell {
    fn new(
        value: char,
        //cost: Option<i32>,
        //action: Option<String>,
        visited: bool,
        state: (usize, usize),
        prev: Option<(usize, usize)>,
        in_path: bool,
    ) -> Cell {
        Cell {
            value,
            //cost,
            //action,
            visited,
            state,
            prev,
            in_path,
        }
    }
    fn set_val(&mut self, val: char) -> () {
        self.value = val;
    }
    fn set_state(&mut self, coords: (usize, usize)) -> () {
        self.state = coords;
    }
}

fn is_cell_goal(params: &Config, cell: &Cell) -> bool {
    (params.goal_x, params.goal_y) == cell.state
}

fn expand(params: &Config, map: &mut Map, cell: &Cell) -> VecDeque<Cell> {
    let (state_x, state_y) = cell.state;
    let mut expanded_cells: VecDeque<Cell> = VecDeque::new();
    //move right
    if state_x + 1 < map.width
        && !map.map[state_x + 1][state_y].visited
        && map.map[state_x + 1][state_y].value != 'W'
    {
        map.map[state_x + 1][state_y].visited = true;
        map.map[state_x + 1][state_y].prev = Some(cell.state);
        expanded_cells.push_back(map.map[state_x + 1][state_y].clone());
    }
    //move left
    if state_x > 1
        && !map.map[state_x - 1][state_y].visited
        && map.map[state_x - 1][state_y].value != 'W'
    {
        map.map[state_x - 1][state_y].visited = true;
        map.map[state_x - 1][state_y].prev = Some(cell.state);

        expanded_cells.push_back(map.map[state_x - 1][state_y].clone());
    }
    //move down
    if state_y + 1 < map.height
        && !map.map[state_x][state_y + 1].visited
        && map.map[state_x][state_y + 1].value != 'W'
    {
        map.map[state_x][state_y + 1].visited = true;
        map.map[state_x][state_y + 1].prev = Some(cell.state);
        expanded_cells.push_back(map.map[state_x][state_y + 1].clone());
    }
    //move up
    if state_y > 1
        && !map.map[state_x][state_y - 1].visited
        && map.map[state_x][state_y - 1].value != 'W'
    {
        map.map[state_x][state_y - 1].visited = true;
        map.map[state_x][state_y - 1].prev = Some(cell.state);

        expanded_cells.push_back(map.map[state_x][state_y - 1].clone());
    }
    expanded_cells
}

fn breadth_first_search(params: Config, map: &mut Map) -> Option<Cell> {
    println!("Performing breadth first search!");
    let mut node = map.map[params.start_x][params.start_y].clone();
    if is_cell_goal(&params, &node) {
        return Some(node);
    }
    let mut expand_cells: VecDeque<Cell> = VecDeque::new();
    node.visited = true;
    expand_cells.push_back(node);
    while !expand_cells.is_empty() {
        node = expand_cells.pop_front()?;
        for cell in expand(&params, map, &node) {
            expand_cells.push_back(cell.clone());
            if is_cell_goal(&params, &cell) {
                return Some(cell);
            }
        }
    }
    return None;
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{}", std::env::current_dir().unwrap().display());
    let mut map = Map::create(&config.file_path)?;
    if let Some(cell) = breadth_first_search(config, &mut map) {
        println!("Solution found!");
        println!("{:?}", cell.state);
        map.set_path(cell);
    } else {
        println!("Solution not found.");
    }
    map.print();

    map.print_path();

    Ok(())
}
