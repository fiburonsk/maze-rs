use shared::Progress;
use std::env;

mod backtracker;
mod img;
mod maze;
mod prims;
mod prims2;
mod psolver;
mod shared;
mod solver;

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
    let mut args = env::args();
    args.next();
    let seed = args
        .next()
        .map(|s| s.parse::<usize>().unwrap_or(1))
        .unwrap_or(1);
    let height = args
        .next()
        .map(|s| s.parse::<usize>().unwrap_or(11))
        .unwrap_or(11);
    let width = args
        .next()
        .map(|s| s.parse::<usize>().unwrap_or(11))
        .unwrap_or(11);
    let show_build = args
        .next()
        .map(|s| match s.parse::<u64>() {
            Ok(x) if x > 0 => Progress::Delay(x),
            _ => Progress::None,
        })
        .unwrap_or(Progress::None);
    let show_solve = args
        .next()
        .map(|s| match s.parse::<u64>() {
            Ok(x) if x > 0 => Progress::Delay(x),
            _ => Progress::None,
        })
        .unwrap_or(Progress::None);

    let maze = match &args.next() {
        Some(x) if x == "p" => prims::generate(seed, height, width, show_build.clone()),
        Some(x) if x == "p2" => prims2::generate(seed, height, width, show_build.clone()),
        _ => backtracker::generate(seed, height, width, show_build.clone()),
    };

    let message = format!(
        "Here is the maze: [seed: {}, height: {}, width: {}]",
        &seed, &height, &width
    );

    // if let Some(solution) = solver::solve(&maze, &show_solve) {
    //     // shared::draw_reset();
    //     // shared::clear_screen();
    //     println!("{}", &message);
    //     // maze::print_maze(&maze);
    //     println!();
    //     // print_maze_with_solution(&maze, &solution);

    //     img::save(&maze, &solution, "maze.png");
    // } else {
    //     shared::clear_screen();
    //     println!("{}", &message);
    //     maze::print_maze(&maze);
    //     println!();
    //     println!("Unable to solve the maze.");
    // }

    if let Some(solution) = psolver::solve(&maze, &show_solve) {
        println!("{}", &message);
        img::save(&maze, &solution, "maze.png");
    } else {
        println!("Unable to solve the maze.");
    }
}
