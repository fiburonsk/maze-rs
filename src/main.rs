use argh::FromArgs;
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

#[derive(Debug, FromArgs)]
/// Maze solving application.
struct CommandLine {
    #[argh(
        option,
        description = "seed used to build the maze [default: 1]",
        short = 'r',
        default = "1"
    )]
    seed: usize,

    #[argh(
        option,
        description = "height [default: 11]",
        short = 'h',
        default = "11"
    )]
    height: usize,

    #[argh(
        option,
        description = "width [default: 11]",
        short = 'w',
        default = "11"
    )]
    width: usize,

    #[argh(
        option,
        description = "solve speed use 0 to hide [default: 0]",
        short = 's',
        default = "0"
    )]
    solve_speed: u64,

    #[argh(
        option,
        description = "build speed use 0 to hide [default: 0]",
        short = 'b',
        default = "0"
    )]
    build_speed: u64,

    #[argh(
        option,
        description = "solving method: [backtracker, prims, prims-backtracker] [default: backtracker]",
        short = 'm',
        default = "Strategy::Backtracker"
    )]
    method: Strategy,

    #[argh(option, description = "save result to image", short = 'i')]
    image: Option<String>,
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

fn main() {
    let matches: CommandLine = argh::from_env();

    let show_build = match matches.build_speed {
        0 => Progress::None,
        _ => Progress::Delay(matches.build_speed),
    };
    let show_solve = match matches.solve_speed {
        0 => Progress::None,
        _ => Progress::Delay(matches.solve_speed),
    };

    let maze = match matches.method {
        Strategy::Backtracker => {
            backtracker::generate(matches.seed, matches.height, matches.width, show_build)
        }
        Strategy::Prims => prims::generate(matches.seed, matches.height, matches.width, show_build),
        Strategy::PrimsBacktracker => {
            prims2::generate(matches.seed, matches.height, matches.width, show_build)
        }
    };

    if let Some(solution) = solver::solve(&maze, &show_solve) {
        if let Progress::Delay(_t) = &show_solve {
            shared::draw_reset();
            shared::clear_screen();
            println!(
                "Maze: [seed: {}, height: {}, width: {}]",
                &matches.seed, &matches.height, &matches.width
            );
            maze::print_maze(&maze);
            println!();
            print_maze_with_solution(&maze, &solution);
        }

        if let Some(image) = matches.image {
            img::save(&maze, &solution, &image);
        }
    } else {
        println!("Unable to solve the maze.");
    }
}
