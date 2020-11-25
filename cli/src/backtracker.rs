use super::print;
use maze_lib::{
    backtracker as lib_backtracker,
    maze::{Blocks, Maze, Part, Pos},
    shared::{self, ChangeBoard, Progress},
};
use rand::{rngs::StdRng, SeedableRng};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn generate(seed: usize, height: usize, width: usize, progress: Progress) -> Maze {
    if let Progress::Delay(_) = progress {
        print::clear_screen();
    }

    let mut maze = Maze::new_empty(height, width);
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed as u64);
    let first = Pos { x: 0, y: 1 };
    let start = Pos { x: 1, y: 1 };
    let mut visited: Blocks = vec![start.clone()];
    maze.open(&first);
    maze.open(&start);

    print::draw_board(&maze, &progress);

    while let Some(current) = visited.pop() {
        if let Some((wall, next)) = lib_backtracker::step(&maze, &mut rng, &current) {
            maze.open(&wall);
            maze.open(&next);

            if let Progress::Delay(time) = progress {
                print::print_part(&wall, &maze);
                print::print_part(&next, &maze);
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_micros(time));
            }

            visited.push(current);
            visited.push(next);
        }
    }

    maze.change(&first, Part::Start);
    maze.change(&shared::pick_end(&mut rng, &maze), Part::Finish);

    maze
}
