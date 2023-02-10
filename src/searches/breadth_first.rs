use crate::structure::map::Map;
use std::collections::VecDeque;
//fil contains logic for breadth first search

pub fn breadth_first(map: &mut Map) -> Option<()> {
    println!("Breadth First Search:\n-----------------------");
    let instance = std::time::Instant::now();
    let mut point = map.start;
    if map.goal == point {
        map.exec_time = instance.elapsed().as_nanos();
        map.set_path(point);
        return Some(());
    }
    let mut expand_points: VecDeque<(usize, usize)> = VecDeque::new();
    map.get(&point).path_cost = 1;
    expand_points.push_back(point);
    while !expand_points.is_empty() {
        point = expand_points.pop_front()?;
        for expand_pt in map.expand(&point) {
            let mut cell = map.get(&expand_pt);
            if cell.path_cost != 1 {
                cell.path_cost = 1;
                cell.prev = Some((point.0, point.1));
                expand_points.push_back(expand_pt);
            }
            if map.goal == expand_pt {
                map.exec_time = instance.elapsed().as_nanos();
                map.set_path(expand_pt);
                return Some(());
            }
        }
    }
    return None;
}
