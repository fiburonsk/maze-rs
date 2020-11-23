extern crate web_sys;

mod utils;

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello hello hello, maze-web!");
// }

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into());
    }
}

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
pub struct Loc {
    x: u32,
    y: u32,
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
        }
    }

    pub fn is_built(&self) -> bool {
        self.is_built
    }

    pub fn build_tick(&mut self) {
        if true == self.is_built {
            return;
        }

        if false == self.is_build_started {
            let first = maze::Pos { x: 0, y: 1 };
            let start = maze::Pos { x: 1, y: 1 };
            self.is_build_started = true;
            self.maze.open(&first);
            self.maze.open(&start);
            self.build_visited.push(start.clone());

            return;
        }

        if let Some(pos) = self.build_visited.pop() {
            if let Some((wall, next)) = backtracker::step(&self.maze, &mut self.build_rng, &pos) {
                self.maze.open(&wall);
                self.maze.open(&next);
                self.build_visited.push(pos);
                self.build_visited.push(next);

                return;
            };
        };

        if self.build_visited.is_empty() {
            self.maze
                .change(&maze::Pos { x: 0, y: 1 }, maze::Part::Start);
            self.maze.change(
                &pick_end(&mut self.build_rng, &self.maze),
                maze::Part::Finish,
            );
            self.is_built = true;
        }
    }

    pub fn solve_tick(&self) {
        if false == self.is_built() {
            return;
        }

        // let start = match maze.start_at() {
        //     Some(pos) => pos,
        //     None => return None,
        // };
    }

    pub fn get_seed(&self) -> usize {
        self.seed
    }

    pub fn get_board(&self) -> *const maze::Part {
        self.maze.board.as_ptr()
    }

    pub fn solve(&self) -> Vec<usize> {
        if let Some(sol) = solver::solve(&self.maze) {
            sol
        } else {
            vec![]
        }
    }
}
