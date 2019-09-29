use clap::{self, value_t};
use shared::Progress;
use std::env;
use std::str::FromStr;

mod backtracker;
mod img;
mod maze;
mod prims;
mod prims2;
mod shared;
mod solver;
mod threadpool;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
enum Strategy {
    Backtracker,
    Prims,
    PrimsBacktracker,
}

impl FromStr for Strategy {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "backtracker" => Ok(Strategy::Backtracker),
            "prims" => Ok(Strategy::Prims),
            "prims-backtracker" => Ok(Strategy::PrimsBacktracker),
            _ => Err("no match"),
        }
    }
}

fn print_maze_with_solution(maze: &maze::Maze, solution: &[maze::Pos]) {
    for (y, row) in maze.board.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col != &(maze::Part::Start)
                && col != &(maze::Part::Finish)
                && solution.contains(&(maze::Pos { x: x, y: y }))
            {
                shared::print_visited();
            } else {
                print!("{}", &col);
            }
        }
        println!();
    }
}

fn init_arguments<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("Maze")
        .version(VERSION)
        .about("Maze generation and solving application.")
        .arg(
            clap::Arg::with_name("seed")
                .short("r")
                .long("seed")
                .takes_value(true)
                .default_value("1")
                .help("Seed used by the random number generator to build the maze."),
        )
        .arg(
            clap::Arg::with_name("height")
                .short("h")
                .long("height")
                .takes_value(true)
                .default_value("11")
                .help("Height of maze. Use odd numbers for a wall on the border."),
        )
        .arg(
            clap::Arg::with_name("width")
                .short("w")
                .long("width")
                .takes_value(true)
                .default_value("11")
                .help("Width of maze, Use odd numbers for a wall on the border."),
        )
        .arg(
            clap::Arg::with_name("method")
                .short("m")
                .long("method")
                .takes_value(true)
                .possible_values(&["backtracker", "prims", "prims-backtracker"])
                .default_value("backtracker")
                .help("Maze generation method."),
        )
        .arg(
            clap::Arg::with_name("build")
                .short("b")
                .long("build-speed")
                .takes_value(true)
                .default_value("0")
                .help("Set the build speed. Lower is faster. A value of zero will not show building the maze."),
        )
        .arg(
            clap::Arg::with_name("solve")
                .short("s")
                .long("solve-speed")
                .takes_value(true)
                .default_value("0")
                .help("Set the solve speed. Lower is faster. A value of zero will not show solving the maze."),
        )
        .arg(
            clap::Arg::with_name("image")
                .short("i")
                .long("image")
                .takes_value(true)
                .help("Save the solved maze as a .png image.  Provide the path and filename to save an image."),
        )
        .get_matches()
}

fn main() {
    let matches = init_arguments();

    let seed = value_t!(matches, "seed", usize).unwrap_or(1);
    let height = value_t!(matches, "height", usize).unwrap_or(11);
    let width = value_t!(matches, "width", usize).unwrap_or(11);

    let show_build = match value_t!(matches, "build", u64) {
        Ok(x) if x > 0 => Progress::Delay(x),
        _ => Progress::None,
    };

    let show_solve = match value_t!(matches, "solve", u64) {
        Ok(x) if x > 0 => Progress::Delay(x),
        _ => Progress::None,
    };

    let maze = match value_t!(matches, "method", Strategy).unwrap_or(Strategy::Backtracker) {
        Strategy::Backtracker => backtracker::generate(seed, height, width, show_build),
        Strategy::Prims => prims::generate(seed, height, width, show_build),
        Strategy::PrimsBacktracker => prims2::generate(seed, height, width, show_build),
    };

    if let Some(solution) = solver::solve(&maze, &show_solve) {
        if let Progress::Delay(_t) = &show_solve {
            shared::draw_reset();
            shared::clear_screen();
            println!(
                "Maze: [seed: {}, height: {}, width: {}]",
                &seed, &height, &width
            );
            maze::print_maze(&maze);
            println!();
            print_maze_with_solution(&maze, &solution);
        }

        if let Some(image) = &matches.value_of("image") {
            img::save(&maze, &solution, image);
        }
    } else {
        println!("Unable to solve the maze.");
    }
}
