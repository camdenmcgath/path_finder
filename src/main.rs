use crate::searches::{
    a_star::{a_star_manhatten, a_star_manhatten_weighted},
    breadth_first::breadth_first,
    dijkstra::dijkstra,
    iterative_deepening::iterative_deepening,
};
use crate::structure::map::Map;
use std::env;
use std::error::Error;
use std::process;
pub mod searches;
pub mod structure;

fn main() {
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
            if let Some(_) = breadth_first(map) {
                map.print_path();
            } else {
                println!("Goal not found...");
            }
        }
        "-l" => {
            if let Some(_) = dijkstra(map) {
                map.print_path();
            } else {
                println!("Goal not found...");
            }
        }
        "-i" | "-ia" => {
            if let Some(_) = iterative_deepening(map) {
                map.print_path();
            } else {
                println!("Goal not found...");
            }
        }
        "-a1" => {
            if let Some(_) = a_star_manhatten(map) {
                map.print_path();
            } else {
                println!("Goal not found...");
            }
        }
        "-a2" => {
            if let Some(_) = a_star_manhatten_weighted(map, 2) {
                map.print_path();
            } else {
                println!("Goal ot found...");
            }
        }
        "-a3" => {
            if let Some(_) = a_star_manhatten_weighted(map, 3) {
                map.print_path();
            } else {
                println!("Goal ot found...");
            }
        }
        "-a4" => {
            if let Some(_) = a_star_manhatten_weighted(map, 4) {
                map.print_path();
            } else {
                println!("Goal ot found...");
            }
        }
        "-a5" => {
            if let Some(_) = a_star_manhatten_weighted(map, 5) {
                map.print_path();
            } else {
                println!("Goal ot found...");
            }
        }
        _ => println!("Please pass one of -b, -l, -i, -a1, -a2 to specify search"),
    }

    Ok(())
}
