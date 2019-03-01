use super::maze::{Maze, Part, Pos};
use super::shared::{self, ChangeBoard, Direction, Movement, Progress};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::collections::VecDeque;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn pick_neighbor(pos: &Pos, m: &Maze, rng: &mut StdRng) -> Option<Direction> {
    let mut directions = shared::all_directions();
    directions.shuffle(rng);
    directions.into_iter().find(|dir| {
        m.go(pos, dir)
            .and_then(|w| m.go(&w, dir))
            .and_then(|c| if !m.is_open(&c) { Some(()) } else { None })
            .is_some()
    })
}

pub fn generate(seed: usize, height: usize, width: usize, progress: Progress) -> Maze {
    shared::clear_screen();
    let mut maze = Maze::new_empty(height, width);
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed as u64);
    let start = Pos { x: 1, y: 1 };
    let mut visited: VecDeque<Pos> = VecDeque::default();
    visited.push_back(start.clone());
    maze.open(&start);
    let mut last = start.clone();

    shared::draw_board(&maze, &progress);

    while let Some(current) = visited.pop_back() {
        if let Some(dir) = pick_neighbor(&current, &maze, &mut rng) {
            let wall = maze.go(&current, &dir).unwrap();
            maze.open(&wall);
            let next = maze.go(&wall, &dir).unwrap();
            maze.open(&next);
            last = next.clone();

            if let Progress::Delay(time) = progress {
                shared::print_part(&wall, &maze);
                shared::print_part(&next, &maze);
                io::stdout().flush().is_ok();
                thread::sleep(Duration::from_micros(time));
            }

            visited.push_front(current);
            visited.push_back(next);
        }
    }

    maze.change(&start, Part::Start);
    maze.change(&last, Part::Finish);

    maze
}
