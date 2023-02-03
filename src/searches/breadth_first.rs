use crate::structure::cell;
use crate::structure::{cell::Cell, direction::Direction, map::Map};
use crate::Config;
use std::collections::VecDeque;
//fil contains logic for breadth first search

pub fn breadth_first(params: &Config, map: &mut Map) -> Option<(usize, usize)> {
    println!("Breadth First Search:\n-----------------------");
    let mut point = params.start;
    if params.goal == point {
        return Some(point);
    }
    let mut expand_points: VecDeque<(usize, usize)> = VecDeque::new();
    map.get(&point).weight = 1;
    expand_points.push_back(point);
    while !expand_points.is_empty() {
        point = expand_points.pop_front()?;
        for expand_pt in map.expand(&point) {
            if map.get(&expand_pt).weight != 1 {
                map.get(&expand_pt).weight = 1;
                map.get(&expand_pt).prev = Some((point.0, point.1));
                map.get(&expand_pt).prev_direction = map.find_prev_direction(&point, &expand_pt);
                expand_points.push_back(expand_pt);
            }
            if params.goal == expand_pt {
                let mut explored = 0;
                map.map.iter().for_each(|row| {
                    row.iter().for_each(|cell| {
                        if cell.weight == 1 {
                            explored += 1;
                        }
                    })
                });
                map.explored = explored;
                return Some(expand_pt);
            }
        }
    }
    return None;
}
