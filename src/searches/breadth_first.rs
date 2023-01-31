use crate::structure::map;
use crate::structure::{cell::Cell, direction::Direction, map::Map};
use crate::Config;
use std::collections::VecDeque;
//fil contains logic for breadth first search
pub fn is_cell_goal(params: &Config, point: &(usize, usize)) -> bool {
    (params.goal_x, params.goal_y) == *point
}

pub fn find_prev_direction(
    map: &Map,
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
    else if origin_x + 1 < map.width && point_x == origin_x + 1 {
        Some(Direction::Left)
    }
    //moved down
    else if origin_y + 1 < map.height && point_y == origin_y + 1 {
        Some(Direction::Up)
    }
    //moved left
    else if origin_x > 0 && point_x == origin_x - 1 {
        Some(Direction::Right)
    } else {
        None
    }
}

pub fn expand(map: &mut Map, point: &(usize, usize)) -> VecDeque<(usize, usize)> {
    let (state_x, state_y) = *point;
    let mut expanded_cells: VecDeque<(usize, usize)> = VecDeque::new();
    //move up
    if state_y > 0
        //&& map.map[state_y - 1][state_x].cost == usize::MAX
        && map.map[state_y - 1][state_x].value != 'W'
    {
        //map.map[state_y - 1][state_x].cost = 1;
        //map.map[state_y - 1][state_x].prev = Some(*point);
        //map.map[state_y - 1][state_x].prev_direction = Some(Direction::Down);
        expanded_cells.push_back(map.map[state_y - 1][state_x].state);
    }
    //move right
    if state_x + 1 < map.width
        //&& map.map[state_y][state_x + 1].cost == usize::MAX
        && map.map[state_y][state_x + 1].value != 'W'
    {
        //map.map[state_y][state_x + 1].cost = 1;
        //map.map[state_y][state_x + 1].prev = Some(*point);
        //map.map[state_y][state_x + 1].prev_direction = Some(Direction::Left);
        expanded_cells.push_back(map.map[state_y][state_x + 1].state);
    }
    //move down
    if state_y + 1 < map.height
        //&& map.map[state_y + 1][state_x].cost == usize::MAX
        && map.map[state_y + 1][state_x].value != 'W'
    {
        //map.map[state_y + 1][state_x].cost = 1;
        //map.map[state_y + 1][state_x].prev = Some(*point);
        //map.map[state_y + 1][state_x].prev_direction = Some(Direction::Up);
        expanded_cells.push_back(map.map[state_y + 1][state_x].state);
    }
    //move left
    if state_x > 0
        //&& map.map[state_y][state_x - 1].cost == usize::MAX
        && map.map[state_y][state_x - 1].value != 'W'
    {
        //map.map[state_y][state_x - 1].cost = 1;
        //map.map[state_y][state_x - 1].prev = Some(*point);
        //map.map[state_y][state_x - 1].prev_direction = Some(Direction::Right);
        expanded_cells.push_back(map.map[state_y][state_x - 1].state);
    }
    expanded_cells
}

pub fn breadth_first_search(params: &Config, map: &mut Map) -> Option<Cell> {
    println!("Breadth First Search:\n-----------------------");
    let mut point = (params.start_x, params.start_y);
    if is_cell_goal(&params, &point) {
        return Some(map.map[params.start_y][params.start_x].clone());
    }
    let mut expand_points: VecDeque<(usize, usize)> = VecDeque::new();
    map.map[params.start_y][params.start_x].cost = 1;
    expand_points.push_back(point);
    while !expand_points.is_empty() {
        point = expand_points.pop_front()?;
        for (x, y) in expand(map, &point) {
            if map.map[y][x].cost != 1 {
                map.map[y][x].cost = 1;
                map.map[y][x].prev = Some((point.0, point.1));
                map.map[y][x].prev_direction = find_prev_direction(&map, &point, &(x, y));
                expand_points.push_back((x, y));
            }
            if is_cell_goal(&params, &(x, y)) {
                return Some(map.map[y][x].clone());
            }
        }
    }
    return None;
}
