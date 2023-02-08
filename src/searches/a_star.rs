//manahtten distance A* search

use crate::structure::{cell::Cell, map::Map};
use std::collections::{BinaryHeap, VecDeque};

pub fn a_star_manhatten(map: &mut Map) -> Option<VecDeque<(usize, usize)>> {
    println!("A* Search With Manahtten Distance Heuristic:\n-----------------------");
    let instance = std::time::Instant::now();
    let mut pq = BinaryHeap::<Cell>::new();
    let start = map.start;
    map.get(&start).weight = 0;
    map.get(&start).path_cost = 0;
    pq.push(map.get(&start).clone());

    while let Some(point) = pq.pop() {
        if map.goal == point.state {
            map.exec_time = instance.elapsed().as_nanos();
            return Some(map.set_path(point.state));
        }
        let goal = map.goal;
        for coords in map.expand(&point.state) {
            let mut cell = map.get(&coords);
            if cell.path_cost == usize::MAX
                || cell.path_cost + cell.weight + manahtten_dist(&cell.state, &goal)
                    < cell.estimated_cost
            {
                cell.prev = Some(point.state);
                cell.path_cost = point.path_cost + cell.weight;
                cell.estimated_cost =
                    point.path_cost + cell.weight + manahtten_dist(&cell.state, &goal);
                pq.push(cell.clone());
            }
        }
    }
    None
}

fn manahtten_dist(start: &(usize, usize), goal: &(usize, usize)) -> usize {
    start.0.abs_diff(goal.0) + start.1.abs_diff(goal.1)
}

pub fn a_star_manhatten_weighted(map: &mut Map, weight: usize) -> Option<VecDeque<(usize, usize)>> {
    println!("A* Search With Weighted Manhatten Heuristic:\n-----------------------");
    let instance = std::time::Instant::now();
    let mut pq = BinaryHeap::<Cell>::new();
    let start = map.start;
    map.get(&start).weight = 0;
    map.get(&start).path_cost = 0;
    pq.push(map.get(&start).clone());

    while let Some(point) = pq.pop() {
        if map.goal == point.state {
            map.exec_time = instance.elapsed().as_nanos();
            return Some(map.set_path(point.state));
        }
        let goal = map.goal;
        for coords in map.expand(&point.state) {
            let mut cell = map.get(&coords);
            if cell.path_cost == usize::MAX
                || cell.path_cost + cell.weight + weight * manahtten_dist(&cell.state, &goal)
                    < cell.estimated_cost
            {
                cell.prev = Some(point.state);
                cell.path_cost = point.path_cost + cell.weight;
                cell.estimated_cost =
                    point.path_cost + cell.weight + weight * manahtten_dist(&cell.state, &goal);

                pq.push(cell.clone());
            }
        }
    }
    None
}
