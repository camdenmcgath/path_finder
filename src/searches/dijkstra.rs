use crate::structure::{cell::Cell, cost_point::CostPoint, map::Map};
use crate::Config;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::fs::read;

//TODO: add correct output of path length, cost, cells explored
pub fn dijkstra(params: &Config, map: &mut Map) -> Option<(usize, usize)> {
    println!("Lowest Cost Search Using Djikstra's Algorithm:\n-----------------------");
    map.set_cell_costs();
    let mut visited = HashMap::<(usize, usize), usize>::new();
    let mut pq = BinaryHeap::<CostPoint>::new();
    map.get(&params.start).weight = 0;
    visited.insert(params.start, 0);
    pq.push(CostPoint {
        cost: 0,
        pt: params.start,
    });

    while let Some(point) = pq.pop() {
        if params.goal == point.pt {
            map.solution_cost = visited[&point.pt];
            map.explored = visited.len();
            return Some(point.pt);
        }
        for coords in map.expand(&point.pt) {
            let mut cell = map.get(&coords);
            if !visited.contains_key(&coords)
                || visited[&point.pt] + map.get(&coords).weight < visited[&coords]
            {
                visited.insert(coords, visited[&point.pt] + map.get(&coords).weight);
                map.get(&coords).prev = Some(point.pt);
                map.get(&coords).prev_direction = map.find_prev_direction(&point.pt, &coords);
                pq.push(CostPoint {
                    cost: visited[&point.pt] + map.get(&coords).weight,
                    pt: coords,
                })
            }
        }
    }
    None
}
