use crate::structure::{cell::Cell, map::Map};
use std::collections::{BinaryHeap, HashMap, VecDeque};

//TODO: add correct output of path length, cost, cells explored
pub fn dijkstra(map: &mut Map) -> Option<VecDeque<(usize, usize)>> {
    println!("Lowest Cost Search Using Djikstra's Algorithm:\n-----------------------");
    let mut visited = HashMap::<(usize, usize), usize>::new();
    let mut pq = BinaryHeap::<Cell>::new();
    let start = map.start;
    map.get(&start).weight = 0;
    map.get(&start).path_cost = 0;
    visited.insert(map.start, 0);
    pq.push(map.get(&start).clone());

    while let Some(point) = pq.pop() {
        if map.goal == point.state {
            return Some(map.set_path(point.state));
        }
        for coords in map.expand(&point.state) {
            let mut cell = map.get(&coords);
            if !visited.contains_key(&coords)
                || visited[&point.state] + cell.weight < visited[&coords]
            {
                visited.insert(coords, visited[&point.state] + cell.weight);
                cell.prev = Some(point.state);
                cell.path_cost = visited[&point.state] + cell.weight;
                pq.push(cell.clone());
            }
        }
    }
    None
}
