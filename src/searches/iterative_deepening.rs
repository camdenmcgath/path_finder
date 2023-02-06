use crate::structure::cell::Cell;
use crate::structure::map;
use crate::structure::{cost_point::CostPoint, map::Map};
use std::collections::{HashMap, VecDeque};
use std::vec::Vec;
use std::{result, vec};

pub fn iterative_deepening(map: &mut Map) -> Option<VecDeque<(usize, usize)>> {
    for cost in (70..=250).step_by(30) {
        if let Some(result) = cost_limited(map, cost) {
            return Some(map.set_path(result));
        }
    }
    None
}

fn cost_limited(map: &mut Map, cost_limit: usize) -> Option<(usize, usize)> {
    let mut stack = Vec::<Cell>::new();
    let mut cycle_stack = Vec::<Cell>::new();
    map.get(&map.start.clone()).path_cost = 0;
    stack.push(map.get(&map.start.clone()).clone());
    while let Some(point) = stack.pop() {
        if map.goal == point.state {
            return Some(point.state);
        }
        if point.path_cost > cost_limit {
            continue;
        } else if !is_cycle(&mut stack, &point) {
            cycle_stack.push(point.clone());
            for coords in map.expand(&point.state) {
                {
                    let mut cell = map.get(&coords);
                    cell.prev = Some(point.state);
                    cell.path_cost = point.path_cost + cell.weight;
                    stack.push(cell.clone());
                }
            }
        }
    }
    None
}

fn is_cycle(stack: &mut Vec<Cell>, point: &Cell) -> bool {
    if let Some(last) = point.prev {
        while let Some(top) = stack.last() {
            if top.state == last {
                return true;
            } else {
                stack.pop();
            }
        }
    }
    false
}
