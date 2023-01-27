use crate::map::{Cell, Direction, Map};
use crate::Config;
use std::collections::VecDeque;
//fil contains logic for breadth first search
fn is_cell_goal(params: &Config, cell: &Cell) -> bool {
    (params.goal_x, params.goal_y) == cell.state
}

fn expand(map: &mut Map, cell: &Cell) -> VecDeque<Cell> {
    let (state_x, state_y) = cell.state;
    let mut expanded_cells: VecDeque<Cell> = VecDeque::new();
    //move up
    if state_y > 1
        && !map.map[state_y - 1][state_x].visited
        && map.map[state_y - 1][state_x].value != 'W'
    {
        map.cells_explored += 1;
        map.map[state_y - 1][state_x].visited = true;
        map.map[state_y - 1][state_x].prev = Some(cell.state);
        map.map[state_y - 1][state_x].prev_direction = Some(Direction::Down);
        expanded_cells.push_back(map.map[state_y - 1][state_x].clone());
    }
    //move right
    if state_x + 1 < map.width
        && !map.map[state_y][state_x + 1].visited
        && map.map[state_y][state_x + 1].value != 'W'
    {
        map.cells_explored += 1;
        map.map[state_y][state_x + 1].visited = true;
        map.map[state_y][state_x + 1].prev = Some(cell.state);
        map.map[state_y][state_x + 1].prev_direction = Some(Direction::Left);
        expanded_cells.push_back(map.map[state_y][state_x + 1].clone());
    }
    //move down
    if state_y + 1 < map.height
        && !map.map[state_y + 1][state_x].visited
        && map.map[state_y + 1][state_x].value != 'W'
    {
        map.cells_explored += 1;
        map.map[state_y + 1][state_x].visited = true;
        map.map[state_y + 1][state_x].prev = Some(cell.state);
        map.map[state_y + 1][state_x].prev_direction = Some(Direction::Up);
        expanded_cells.push_back(map.map[state_y + 1][state_x].clone());
    }
    //move left
    if state_x > 1
        && !map.map[state_y][state_x - 1].visited
        && map.map[state_y][state_x - 1].value != 'W'
    {
        map.cells_explored += 1;
        map.map[state_y][state_x - 1].visited = true;
        map.map[state_y][state_x - 1].prev = Some(cell.state);
        map.map[state_y][state_x - 1].prev_direction = Some(Direction::Right);
        expanded_cells.push_back(map.map[state_y][state_x - 1].clone());
    }
    expanded_cells
}

pub fn breadth_first_search(params: &Config, map: &mut Map) -> Option<Cell> {
    println!("Breadth First Search:\n-----------------------");
    map.map[params.start_y][params.start_x].visited = true;
    let mut node = map.map[params.start_y][params.start_x].clone();
    if is_cell_goal(&params, &node) {
        return Some(node);
    }
    let mut expand_cells: VecDeque<Cell> = VecDeque::new();
    expand_cells.push_back(node);
    while !expand_cells.is_empty() {
        node = expand_cells.pop_front()?;
        for cell in expand(map, &node) {
            expand_cells.push_back(cell.clone());
            if is_cell_goal(&params, &cell) {
                return Some(cell);
            }
        }
    }
    return None;
}
