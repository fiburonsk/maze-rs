use rand::{rngs::StdRng, Rng};

use maze_lib::{
    maze::{Maze, Pos},
    shared::{self, Direction, Movement},
};

use std::vec::Vec;

pub enum Path {
    Solved,
    Cell(usize),
    None,
}

#[derive(Debug, PartialEq)]
pub struct Visit {
    moves: Vec<Direction>,
    pub at: Pos,
}

pub fn solve_tick(
    maze: &Maze,
    rng: &mut StdRng,
    visited: &mut Vec<usize>,
    visitor: &mut Vec<Visit>,
) -> Path {
    let current = if visited.is_empty() {
        maze.start_at().and_then(|pos| {
            Some(Visit {
                at: pos,
                moves: shared::all_directions(),
            })
        })
    } else {
        visitor.pop()
    };

    if let Some(mut visit) = current {
        if maze.is_finished(&visit.at) {
            visitor.push(visit);

            return Path::Solved;
        }

        if visit.moves.is_empty() {
            return Path::None;
        }

        let dir = visit
            .moves
            .clone()
            .into_iter()
            .find(|d| match maze.go(&visit.at, d) {
                Some(next) => maze.is_finished(&next),
                _ => false,
            })
            .unwrap_or_else(|| {
                let i = rng.gen::<usize>() % visit.moves.len();
                visit.moves.remove(i)
            });

        let pos = &visit.at.clone();
        visitor.push(visit);

        if let Some(p) = maze.go(&pos, &dir) {
            let index = maze.pos_to_index(&p);
            if !maze.is_wall(&p) && !visited.contains(&index) {
                visited.push(index);

                let next = Visit {
                    moves: shared::all_directions()
                        .into_iter()
                        .filter(|d| {
                            *d != shared::opposite_dir(&dir)
                                && match maze.go(&p, d) {
                                    Some(new) if !maze.is_wall(&new) => true,
                                    _ => false,
                                }
                        })
                        .collect(),
                    at: p,
                };

                visitor.push(next);

                return Path::Cell(index);
            }
        }
    }

    Path::None
}
