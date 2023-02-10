use crate::structure::{cell::Cell, map::Map};
use std::collections::BinaryHeap;
//TODO: add correct output of path length, cost, cells explored
pub fn dijkstra(map: &mut Map) -> Option<()> {
    println!("Lowest Cost Search Using Djikstra's Algorithm:\n-----------------------");
    let instance = std::time::Instant::now();
    let mut pq = BinaryHeap::<Cell>::new();
    let start = map.start;
    map.get(&start).weight = 0;
    map.get(&start).path_cost = 0;
    pq.push(map.get(&start).clone());

    while let Some(point) = pq.pop() {
        if map.goal == point.state {
            map.exec_time = instance.elapsed().as_nanos();
            map.set_path(point.state);
            return Some(());
        }
        for coords in map.expand(&point.state) {
            let mut cell = map.get(&coords);
            if cell.path_cost == usize::MAX || point.path_cost + cell.weight < cell.path_cost {
                cell.prev = Some(point.state);
                cell.path_cost = point.path_cost + cell.weight;
                pq.push(cell.clone());
            }
        }
    }
    None
}
