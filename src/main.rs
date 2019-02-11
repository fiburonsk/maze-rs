use rand;
use shared::{clear_screen, Direction, Progress};
use std::env;
use std::thread;
use std::time::Duration;

mod maze;
mod prims;
mod shared;

trait Movement {
    fn go(&self, pos: &maze::Pos, dir: &Direction) -> Option<maze::Pos>;
}

impl Movement for maze::Maze {
    fn go(&self, pos: &maze::Pos, dir: &Direction) -> Option<maze::Pos> {
        let pos = match dir {
            Direction::Up => pos.up(),
            Direction::Down => pos.down(),
            Direction::Right => pos.right(),
            Direction::Left => pos.left(),
        };

        if !self.is_wall(&pos) {
            Some(pos)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct Visit {
    moves: Vec<Direction>,
    at: maze::Pos,
}

fn all_directions() -> Vec<Direction> {
    return vec![
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
}

fn opposite_dir(dir: &Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

fn solve(maze: &maze::Maze, progress: Progress) -> Option<Vec<maze::Pos>> {
    println!("Solve the maze!");
    let start = match maze.start_at() {
        Some(pos) => pos,
        None => return None,
    };

    let mut visited: Vec<maze::Pos> = vec![start.clone()];
    let mut visitor = vec![Visit {
        at: start.clone(),
        moves: all_directions(),
    }];

    loop {
        if visitor.is_empty() {
            break;
        }

        let route = visitor
            .iter()
            .map(|v| v.at.clone())
            .collect::<Vec<maze::Pos>>();

        if let Progress::Delay(time) = progress {
            clear_screen();
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
                if !visited.contains(&p) {
                    visited.push(p.clone());

                    let next = Visit {
                        at: p,
                        moves: all_directions()
                            .into_iter()
                            .filter(|d| *d != opposite_dir(&dir))
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
                print!(".");
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

    // let maze = prims::generate(seed, height, width, Progress::Delay(100_000));
    let maze = prims::generate(seed, height, width, Progress::None);

    if let Some(solution) = solve(&maze, Progress::None) {
        clear_screen();
        println!("Here is the maze to solve:");
        maze::print_maze(&maze);
        println!();
        print_maze_with_solution(&maze, &solution);
    } else {
        clear_screen();
        println!("Here is the maze to solve:");
        maze::print_maze(&maze);
        println!();

        println!("Unable to solve the maze.");
    }

    println!("{:?}", &seed);
    println!("{:?}", &height);
    println!("{:?}", &width);
}
