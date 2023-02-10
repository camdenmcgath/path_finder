use crate::structure::cell::Cell;
use crate::structure::map::Map;
use std::vec::Vec;
pub fn iterative_deepening(map: &mut Map) -> Option<()> {
    println!("Iterative Deepening by Cost\n");
    match map.search_flag.as_str() {
        "-i" => println!("Not avoiding cycles"),
        "-ia" => println!("Avoiding cycles"),
        _ => println!(),
    };
    let instance = std::time::Instant::now();
    for cost in (10..=1000).step_by(10) {
        if let Some(mut result) = cost_limited(map, cost) {
            map.exec_time = instance.elapsed().as_nanos();
            map.set_path_stack(&mut result);
            return Some(());
        }
    }
    None
}

fn cost_limited(map: &mut Map, cost_limit: usize) -> Option<Vec<(usize, usize)>> {
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
            return Some(path_stack);
        }
        if point.path_cost > cost_limit {
            continue;
        } else {
            for coords in map.expand(&point.state) {
                //check if avoiding states
                if map.search_flag == "-i" || !path_stack.contains(&coords) {
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
