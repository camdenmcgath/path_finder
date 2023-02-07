use crate::structure::cell::Cell;
use crate::structure::map::Map;
use std::collections::VecDeque;
use std::vec::Vec;
pub fn iterative_deepening(map: &mut Map) -> Option<VecDeque<(usize, usize)>> {
    for cost in (70..=10000).step_by(30) {
        if let Some(result) = cost_limited(map, cost) {
            return Some(map.set_path(result));
        }
    }
    None
}

fn cost_limited(map: &mut Map, cost_limit: usize) -> Option<(usize, usize)> {
    let mut main_stack = Vec::<Cell>::new();
    let mut path_stack = Vec::<(usize, usize)>::new();
    map.get(&map.start.clone()).path_cost = 0;
    main_stack.push(map.get(&map.start.clone()).clone());
    while let Some(point) = main_stack.pop() {
        if let Some(prev) = point.prev {
            while let Some(top) = path_stack.last() {
                if *top == prev {
                    break;
                } else {
                    path_stack.pop();
                }
            }
        }
        path_stack.push(point.state);
        if map.goal == point.state {
            return Some(point.state);
        }
        if point.path_cost > cost_limit {
            continue;
        } else {
            for coords in map.expand(&point.state) {
                if !path_stack.contains(&coords) {
                    let mut cell = map.get(&coords);
                    cell.prev = Some(point.state);
                    cell.path_cost = point.path_cost + cell.weight;
                    main_stack.push(cell.clone());
                }
            }
        }
    }
    None
}
