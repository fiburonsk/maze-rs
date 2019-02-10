use std::thread;
use std::time::Duration;
mod maze;

const DELAY: u64 = 40;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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

fn inverse_dir(dir: &Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn solve(maze: &maze::Maze) -> Option<Vec<maze::Pos>> {
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

        clear_screen();
        print_maze_with_solution(maze, &route);
        thread::sleep(Duration::from_millis(DELAY));

        let mut visit = visitor.pop().unwrap();

        if maze.is_finished(&visit.at) {
            visitor.push(visit);

            return Some(route);
        }

        if !visit.moves.is_empty() {
            let dir = visit.moves.pop().unwrap();
            let pos = &visit.at.clone();
            visitor.push(visit);

            if let Some(p) = maze.go(&pos, &dir) {
                if !visited.contains(&p) {
                    visited.push(p.clone());

                    let next = Visit {
                        at: p,
                        moves: all_directions()
                            .into_iter()
                            .filter(|d| *d != inverse_dir(&dir))
                            .collect(),
                    };
                    visitor.push(next);
                }
            }
        }
    }

    None
}

fn print_maze(maze: &maze::Maze) {
    for row in maze.board.iter() {
        for col in row.iter() {
            print!("{}", &col);
        }
        println!();
    }
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
    let maze = maze::Maze {
        board: maze::make(),
    };

    if let Some(solution) = solve(&maze) {
        clear_screen();
        println!("Here is the maze to solve:");
        print_maze(&maze);
        println!();
        print_maze_with_solution(&maze, &solution);
    } else {
        println!("Unable to solve the maze.");
    }
}
