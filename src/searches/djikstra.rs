use crate::structure::{cell::Cell, map::Map};
use crate::Config;
use std::collections::{BinaryHeap, HashMap};
use std::fs::read;

//TODO: Create Point class and make binary heap return lowest value always

pub fn djikstra_search(params: &Config, map: &mut Map) -> Option<(usize, usize)> {
    println!("Lowest Cost Search Using Djikstra's Algorithm:\n-----------------------");
    map.set_cell_costs();
    let mut reached_pts = HashMap::<(usize, usize), (usize, usize)>::new();
    let mut p_que = BinaryHeap::<(usize, usize)>::new();
    reached_pts.insert(params.start, params.start);
    p_que.push(params.start);
    while let Some(point) = p_que.pop() {
        if params.goal == point {
            reached_pts
                .iter()
                .for_each(|pair| map.get(pair.0).prev = Some(*pair.1));
            return Some(point);
        }
        for coords in map.expand(&point) {
            if !reached_pts.contains_key(&coords)
                || map.get(&coords).cost < map.get(reached_pts.get(&coords).unwrap()).cost
            {
                map.get(&coords).cost += map.get(&point).cost;
                map.get(&coords).prev_direction = map.find_prev_direction(&point, &coords);
                reached_pts.insert(coords, point);
                p_que.push(coords);
            }
        }
    }
    None
}
