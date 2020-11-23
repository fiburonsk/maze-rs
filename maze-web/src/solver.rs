use rand::{rngs::StdRng, Rng};

use maze_lib::{
    maze::{Blocks, Maze, Pos},
    shared::{self, Direction, Movement},
};

use std::vec::Vec;

enum Run {
    Start(Pos),
    Solve(Branch),
    Solution(Blocks),
}

struct Branch {
    at: Pos,
    dir: Direction,
    path: Vec<Pos>,
}

#[derive(Debug, PartialEq)]
struct Visit {
    moves: Vec<Direction>,
    at: Pos,
}

pub fn solve(maze: &Maze) -> Option<Vec<usize>> {
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
        if maze.is_finished(&visit.at) {
            visitor.push(visit);

            return Some(visitor.iter().map(|v| maze.pos_to_index(&v.at)).collect());
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

// pub fn solve(maze: &Maze) -> Option<Blocks> {
//     let start = match maze.start_at() {
//         Some(pos) => pos,
//         None => return None,
//     };

//     let mut moves = begin(start, maze);
//     let mut visited: Blocks = vec![];

//     while false == moves.is_empty() {
//         let next = match moves.pop() {
//             Some(branch) => branch,
//             None => break,
//         };

//         maze.go(next.at, next.dir);
//     }
// }

// fn run(maze: &Maze) -> Option<Blocks> {
//     let start = match maze.start_at() {
//         Some(pos) => pos,
//         None => return None,
//     };

//     let mut status = Run::Start(start);

//     loop {
//         match status {
//             Run::Start(pos) => {
//                 status = begin(pos, maze);
//             }
//             Run::Solve(branch) => {
//                 status = solver(branch, maze);
//             }
//             Run::Solution(path) => {
//                 break;
//             }
//         };
//     }

//     match status {
//         Run::Solution(path) => Some(path),
//         _ => None,
//     }
// }

fn begin(start: Pos, maze: &Maze) -> Vec<Branch> {
    shared::all_directions()
        .into_iter()
        .filter(|d| {
            maze.go(&start, &d)
                .and_then(|p| if !maze.is_wall(&p) { Some(()) } else { None })
                .is_some()
        })
        .map(|d| Branch {
            at: start.clone(),
            dir: d,
            path: vec![start.clone()],
        })
        .collect()
}

// fn solver(branch: Branch, tx: mpsc::Sender<Run>, maze: &Maze) {
//     let mut at = branch.at.clone();
//     let mut dir = branch.dir;
//     let path = branch.path.clone();

//     let mut visited: Blocks = vec![];

//     while let Some(next) = maze.go(&at, &dir) {
//         at = next;
//         visited.push(at.clone());

//         if maze.is_finished(&at) {
//             let mut new_path = path.clone();
//             new_path.append(&mut visited.clone());
//             tx.send(Run::Solution(new_path)).unwrap();
//         }

//         let mut moves: Vec<Direction> = shared::all_directions()
//             .into_iter()
//             .filter(|d| {
//                 *d != shared::opposite_dir(&dir)
//                     && maze
//                         .go(&at, &d)
//                         .and_then(|p| if !maze.is_wall(&p) { Some(()) } else { None })
//                         .is_some()
//             })
//             .collect();

//         dir = match moves.pop() {
//             Some(p) => p,
//             None => break,
//         };

//         moves.into_iter().for_each(|d| {
//             let mut new_path = path.clone();
//             new_path.append(&mut visited.clone());

//             tx.send(Run::Solve(Branch {
//                 at: at.clone(),
//                 dir: d,
//                 path: new_path,
//             }))
//             .unwrap();
//         });
//     }
// }
