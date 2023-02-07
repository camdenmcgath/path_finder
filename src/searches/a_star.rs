//manahtten distance A* search

use crate::structure::{cell::Cell, map::Map};
use std::collections::{BinaryHeap, HashMap, VecDeque};

pub fn a_star_manhatten(map: &mut Map) -> Option<VecDeque<(usize, usize)>> {
    println!("A* Search With Manahtten Distance Heuristic:\n-----------------------");
    let mut visited = HashMap::<(usize, usize), Cell>::new();
    let mut pq = BinaryHeap::<Cell>::new();
    let start = map.start;
    map.get(&start).weight = 0;
    map.get(&start).path_cost = 0;
    visited.insert(map.start, map.get(&start).clone());
    pq.push(map.get(&start).clone());

    while let Some(point) = pq.pop() {
        if map.goal == point.state {
            return Some(map.set_path(point.state));
        }
        let goal = map.goal;
        for coords in map.expand(&point.state) {
            let mut cell = map.get(&coords);
            if !visited.contains_key(&coords)
                || visited[&point.state].path_cost
                    + cell.weight
                    + manahtten_dist(&cell.state, &goal)
                    < visited[&coords].estimated_cost
            {
                cell.prev = Some(point.state);
                cell.path_cost = visited[&point.state].path_cost + cell.weight;
                cell.estimated_cost = visited[&point.state].path_cost
                    + cell.weight
                    + manahtten_dist(&cell.state, &goal);
                visited.insert(coords, cell.clone());
                pq.push(cell.clone());
            }
        }
    }
    None
}

fn manahtten_dist(start: &(usize, usize), goal: &(usize, usize)) -> usize {
    start.0.abs_diff(goal.0) + start.1.abs_diff(goal.1)
}

pub fn a_start_manhatten_weighted(map: &mut Map) -> Option<VecDeque<(usize, usize)>> {
    println!("A* Search With Weighted Manahtten Distance Heuristic:\n-----------------------");
    let mut visited = HashMap::<(usize, usize), Cell>::new();
    let mut pq = BinaryHeap::<Cell>::new();
    let start = map.start;
    map.get(&start).weight = 0;
    map.get(&start).path_cost = 0;
    visited.insert(map.start, map.get(&start).clone());
    pq.push(map.get(&start).clone());

    while let Some(point) = pq.pop() {
        if map.goal == point.state {
            return Some(map.set_path(point.state));
        }
        let goal = map.goal;
        for coords in map.expand(&point.state) {
            let mut cell = map.get(&coords);
            if !visited.contains_key(&coords)
                || visited[&point.state].path_cost
                    + cell.weight
                    + 2 * manahtten_dist(&cell.state, &goal)
                    < visited[&coords].estimated_cost
            {
                cell.prev = Some(point.state);
                cell.path_cost = visited[&point.state].path_cost + cell.weight;
                cell.estimated_cost = visited[&point.state].path_cost
                    + cell.weight
                    + 2 * manahtten_dist(&cell.state, &goal);
                visited.insert(coords, cell.clone());
                pq.push(cell.clone());
            }
        }
    }
    None
}
