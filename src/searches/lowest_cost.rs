/* use crate::searches::breadth_first::{breadth_first_search, expand, is_cell_goal};
use crate::structure::{cell::Cell, map::Map};
use crate::Config;
use std::collections::VecDeque;

pub fn lowest_cost_search(params: &Config, map: &mut Map) -> Option<Cell> {
    println!("Lowest Cost Search:\n-----------------------");
    map.map[params.start_y][params.start_x].cost = 1;
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
                return Some(cell.clone());
            }
        }
    }
    return None;
}*/
