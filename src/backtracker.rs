use super::maze::{Blocks, Maze, Part, Pos};
use super::shared::{self, ChangeBoard, Direction, Movement, Progress};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
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
    let first = Pos { x: 0, y: 1 };
    let start = Pos { x: 1, y: 1 };
    let mut visited: Blocks = vec![start.clone()];
    maze.open(&first);
    maze.open(&start);

    shared::draw_board(&maze, &progress);

    while let Some(current) = visited.pop() {
        if let Some(dir) = pick_neighbor(&current, &maze, &mut rng) {
            let wall = maze.go(&current, &dir).expect("Should go to direction");
            maze.open(&wall);
            let next = maze.go(&wall, &dir).expect("Should continue in direction");
            maze.open(&next);

            if let Progress::Delay(time) = progress {
                shared::print_part(&wall, &maze);
                shared::print_part(&next, &maze);
                io::stdout().flush().is_ok();
                thread::sleep(Duration::from_micros(time));
            }

            visited.push(current);
            visited.push(next);
        }
    }

    maze.change(&first, Part::Start);
    maze.change(&shared::pick_end(rng.gen::<usize>(), &maze), Part::Finish);

    maze
}
