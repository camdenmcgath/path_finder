use crate::structure::{cost_point::CostPoint, map::Map};
use crate::Config;
use std::collections::HashMap;
use std::vec::Vec;

#[derive(PartialEq)]
enum Result {
    Point((usize, usize)),
    Failure,
    Cutoff,
}

pub fn iterative_deepening(params: &Config, map: &mut Map) -> Option<(usize, usize)> {
    map.set_cell_costs();
    for cost in (70..=220).step_by(30) {
        let result = cost_limited(params, map, cost);
        if result != Result::Cutoff {
            match result {
                Result::Point((x, y)) => return Some((x, y)),
                _ => return None,
            };
        }
    }
    None
}

fn cost_limited(params: &Config, map: &mut Map, cost: usize) -> Result {
    //cost limited portion
    let mut stack = Vec::<CostPoint>::new();
    map.get(&params.start).path_cost = 0;
    stack.push(CostPoint {
        cost: 0,
        pt: params.start,
    });
    while let Some(point) = stack.pop() {
        if params.goal == point.pt {
            return Result::Point(point.pt);
        }
        if point.cost > cost {
            return Result::Cutoff;
        }
        for coords in map.expand(&point.pt) {
            map.get(&coords).prev = Some(point.pt);
            map.get(&coords).prev_direction = map.find_prev_direction(&point.pt, &coords);
            map.get(&coords).path_cost = map.get(&point.pt).path_cost + map.get(&coords).weight;
            stack.push(CostPoint {
                cost: map.get(&point.pt).path_cost + map.get(&coords).weight,
                pt: coords,
            });
        }
    }
    return Result::Failure;
}
