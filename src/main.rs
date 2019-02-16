use maze::Blocks;
use rand;
use shared::{Direction, Movement, Progress};
use std::env;
use std::thread;
use std::time::Duration;

mod backtracker;
mod maze;
mod prims;
mod shared;

#[derive(Debug, PartialEq)]
struct Visit {
    moves: Vec<Direction>,
    at: maze::Pos,
}

fn solve(maze: &maze::Maze, progress: Progress) -> Option<Blocks> {
    println!("Solve the maze!");
    let start = match maze.start_at() {
        Some(pos) => pos,
        None => return None,
    };

    let mut visited: Blocks = vec![start.clone()];
    let mut visitor = vec![Visit {
        at: start.clone(),
        moves: shared::all_directions(),
    }];

    loop {
        if visitor.is_empty() {
            break;
        }

        let route = visitor.iter().map(|v| v.at.clone()).collect::<Blocks>();

        if let Progress::Delay(time) = progress {
            shared::redraw();
            print_maze_with_solution(maze, &route);
            thread::sleep(Duration::from_micros(time));
        }

        let mut visit = visitor.pop().unwrap();

        if maze.is_finished(&visit.at) {
            visitor.push(visit);

            return Some(route);
        }

        if !visit.moves.is_empty() {
            let i = rand::random::<usize>() % visit.moves.len();
            let dir = visit.moves.remove(i);
            let pos = &visit.at.clone();
            visitor.push(visit);

            if let Some(p) = maze.go(&pos, &dir) {
                if !maze.is_wall(&p) && !visited.contains(&p) {
                    visited.push(p.clone());

                    let next = Visit {
                        at: p,
                        moves: shared::all_directions()
                            .into_iter()
                            .filter(|d| *d != shared::opposite_dir(&dir))
                            .collect(),
                    };

                    visitor.push(next);
                }
            }
        }
    }

    None
}

fn print_maze_with_solution(maze: &maze::Maze, solution: &[maze::Pos]) {
    for (y, row) in maze.board.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col != &(maze::Part::Start)
                && col != &(maze::Part::Finish)
                && solution.contains(&(maze::Pos { x: x, y: y }))
            {
                print!("\x1b[0;33m+\x1b[0m");
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
        Some(x) if x == "b" => backtracker::generate(seed, height, width, show_build.clone()),
        _ => prims::generate(seed, height, width, show_build.clone()),
    };

    let message = format!(
        "Here is the maze: [seed: {}, height: {}, width: {}]",
        &seed, &height, &width
    );

    if let Some(solution) = solve(&maze, show_solve) {
        shared::clear_screen();
        println!("{}", &message);
        maze::print_maze(&maze);
        println!();
        print_maze_with_solution(&maze, &solution);
    } else {
        shared::clear_screen();
        println!("{}", &message);
        maze::print_maze(&maze);
        println!();
        println!("Unable to solve the maze.");
    }
}
