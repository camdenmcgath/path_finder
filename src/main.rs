use path_finder::Config;
use std::env;
use std::process;

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
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("File path: {}", config.file_path);
    println!("Starting x,y: {}, {}", config.start.0, config.start.1);
    println!("Goal x,y: {}, {}", config.goal.0, config.goal.1);
    println!("Search flag: {}", config.search_flag);
    if let Err(e) = path_finder::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
