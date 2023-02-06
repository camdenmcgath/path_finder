use crate::searches::{
    breadth_first::breadth_first, dijkstra::dijkstra, iterative_deepening::iterative_deepening,
};
use crate::structure::map::Map;
use std::env;
use std::error::Error;
use std::process;
pub mod searches;
pub mod structure;
//Program to perform a breadth first search on a map of charcters
//given starting and goal coordinates
//main.rs contains the command line arg parsing and initiates the program
//lib.rs contains the code to run the main part of the program
//the breadth first search code is found in the searches module

//TODO: efficiency upgrades: remove in path, remove direction field, check set_path function
//TODO: handle edge cases (coords out of bounds on map), remove redundant calculations
//TODO: refactor grid read in to initialize cost to usize::MAX for water, normal number for
//other based on character, and store a travel_cost value intialized to usize::MAX
//TODO: calculate path direction without storing in field, merge config and map

//Two ways to improve heuristic algorithms: change methods (Beam search, etc.), change heuristic function

fn main() {
    //let args: Vec<String> = env::args().collect();
    let mut map = Map::create(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("File path: {}", map.file_path);
    println!("Starting x,y: {}, {}", map.start.0, map.start.1);
    println!("Goal x,y: {}, {}", map.goal.0, map.goal.1);
    println!("Search flag: {}", map.search_flag);
    if let Err(e) = run(&mut map) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(map: &mut Map) -> Result<(), Box<dyn Error>> {
    println!("{}", std::env::current_dir().unwrap().display());
    match map.search_flag.as_str() {
        "-b" => {
            if let Some(path) = breadth_first(map) {
                map.print_path();
            } else {
                println!("Goal not found...");
            }
        }
        "-l" => {
            if let Some(path) = dijkstra(map) {
                map.print_path();
            } else {
                println!("Oops");
            }
        }
        "-i" => {
            if let Some(path) = iterative_deepening(map) {
                map.print_path();
            } else {
                println!("Oops");
            }
        }
        "-a1" => println!("A* heurstic version 1 not yet implemented"),
        "-a2" => println!("A* huritic version 2 not yet implemented"),
        _ => println!("Please pass one of -b, -l, -i, -a1, -a2 to specify search"),
    }

    Ok(())
}
