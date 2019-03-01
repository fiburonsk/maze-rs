use super::maze::{self, Blocks};
use super::shared::{self, Direction, Movement, Progress};
use rand;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq)]
struct Visit {
    moves: Vec<Direction>,
    at: maze::Pos,
}

pub fn solve(maze: &maze::Maze, progress: &Progress) -> Option<Blocks> {
    if let Progress::Delay(_time) = progress {
        shared::clear_screen();
        shared::draw_board(maze, &progress);
    }

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

    while let Some(mut visit) = visitor.pop() {
        if let Progress::Delay(time) = progress {
            shared::draw_at(&visit.at);
            shared::print_visited();
            io::stdout().flush().is_ok();
            thread::sleep(Duration::from_micros(*time));
        }

        if maze.is_finished(&visit.at) {
            visitor.push(visit);

            return Some(visitor.iter().map(|v| v.at.clone()).collect());
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
        } else if let Progress::Delay(_time) = progress {
            shared::draw_at(&visit.at);
            shared::print_part(&visit.at, &maze);
        }
    }

    None
}
