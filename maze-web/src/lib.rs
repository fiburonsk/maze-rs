extern crate web_sys;

mod utils;

use rand::{rngs::StdRng, SeedableRng};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::error_1(&format!( $( $t )* ).into());
//     }
// }

mod solver;

use maze_lib::{backtracker, maze, shared::pick_end, shared::ChangeBoard};

#[wasm_bindgen]
pub fn get_parts() -> std::vec::Vec<u8> {
    vec![
        maze::Part::Open as u8,
        maze::Part::Wall as u8,
        maze::Part::Start as u8,
        maze::Part::Finish as u8,
    ]
}

#[wasm_bindgen]
pub struct App {
    seed: usize,
    maze: maze::Maze,
    build_visited: Vec<maze::Pos>,
    is_built: bool,
    is_build_started: bool,
    build_rng: StdRng,
    solve_rng: StdRng,
    visited: Vec<usize>,
    visitor: Vec<solver::Visit>,
    is_solved: bool,
}

#[wasm_bindgen]
impl App {
    pub fn new(seed: usize, height: usize, width: usize) -> App {
        utils::set_panic_hook();
        App {
            seed,
            maze: maze::Maze::new_empty(height, width),
            build_visited: vec![],
            is_built: false,
            is_build_started: false,
            build_rng: SeedableRng::seed_from_u64(seed as u64),
            solve_rng: SeedableRng::seed_from_u64(seed as u64),
            visited: vec![],
            visitor: vec![],
            is_solved: false,
        }
    }

    pub fn is_built(&self) -> bool {
        self.is_built
    }

    pub fn is_solved(&self) -> bool {
        self.is_solved
    }

    pub fn build_tick(&mut self) -> Vec<usize> {
        if true == self.is_built {
            return vec![];
        }

        if false == self.is_build_started {
            let first = maze::Pos { x: 0, y: 1 };
            let start = maze::Pos { x: 1, y: 1 };
            self.is_build_started = true;
            self.maze.open(&first);
            self.maze.open(&start);
            self.build_visited.push(start.clone());

            return vec![
                self.maze.pos_to_index(&first),
                self.maze.pos_to_index(&start),
            ];
        }

        if let Some(pos) = self.build_visited.pop() {
            if let Some((wall, next)) = backtracker::step(&self.maze, &mut self.build_rng, &pos) {
                self.maze.open(&wall);
                self.maze.open(&next);

                let new = vec![self.maze.pos_to_index(&wall), self.maze.pos_to_index(&next)];

                self.build_visited.push(pos);
                self.build_visited.push(next);

                return new;
            };
        };

        if self.build_visited.is_empty() {
            let start = maze::Pos { x: 0, y: 1 };
            let end = pick_end(&mut self.build_rng, &self.maze);
            self.maze.change(&start, maze::Part::Start);
            self.maze.change(&end, maze::Part::Finish);

            self.is_built = true;

            return vec![self.maze.pos_to_index(&start), self.maze.pos_to_index(&end)];
        }

        vec![]
    }

    pub fn solve_tick(&mut self) -> Vec<usize> {
        if false == self.is_built() || true == self.is_solved() {
            return vec![];
        }

        match solver::solve_tick(
            &self.maze,
            &mut self.solve_rng,
            &mut self.visited,
            &mut self.visitor,
        ) {
            solver::Path::Cell(idx) => vec![idx],
            solver::Path::Solved => {
                self.is_solved = true;
                return vec![];
            }
            solver::Path::None => vec![],
        }
    }

    pub fn get_seed(&self) -> usize {
        self.seed
    }

    pub fn get_board(&self) -> *const maze::Part {
        self.maze.board.as_ptr()
    }

    pub fn solution(&self) -> Vec<usize> {
        self.visitor
            .iter()
            .map(|v| self.maze.pos_to_index(&v.at))
            .collect()
    }
}
