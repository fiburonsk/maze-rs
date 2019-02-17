use rand::{rngs::StdRng, Rng, SeedableRng};

use super::maze::{Blocks, Maze, Part, Pos};
use super::shared::{self, draw_board, ChangeBoard, Direction, Movement, Progress};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn walls_for(pos: &Pos, m: &Maze) -> Blocks {
    shared::all_directions()
        .iter()
        .filter_map(|dir| m.go(pos, dir))
        .filter(|p| m.is_wall(p))
        .collect::<Blocks>()
}

fn is_wall_horizontal(check: &Pos, start: &Pos) -> bool {
    check.x % 2 != start.x % 2
}

fn find_next(wall: &Pos, m: &Maze, start: &Pos) -> Option<Pos> {
    let cells = if is_wall_horizontal(wall, start) {
        Some((
            m.go(wall, &(Direction::Left)),
            m.go(wall, &(Direction::Right)),
        ))
    } else {
        Some((m.go(wall, &(Direction::Up)), m.go(wall, &(Direction::Down))))
    };

    cells
        .and_then(|c| {
            if c.0.is_some() && c.1.is_some() {
                Some((c.0.unwrap(), c.1.unwrap()))
            } else {
                None
            }
        })
        .and_then(|c| {
            if m.is_open(&c.0) && !m.is_open(&c.1) {
                Some(c.1)
            } else if m.is_open(&c.1) && !m.is_open(&c.0) {
                Some(c.0)
            } else {
                None
            }
        })
}

pub fn generate(seed: usize, height: usize, width: usize, progress: Progress) -> Maze {
    shared::clear_screen();
    let mut maze = Maze::new_empty(height, width);
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed as u64);
    let start = shared::pick_start(rng.gen::<usize>(), rng.gen::<usize>(), height, width);
    let mut frontier: Blocks = vec![start.clone()];
    maze.open(&start);
    let mut last = start.clone();
    let mut walls: Blocks = walls_for(&start, &maze);
    shared::draw_board(&maze, &progress);

    while !walls.is_empty() {
        let wall = {
            let index = rng.gen::<usize>() % walls.len();
            walls.remove(index)
        };

        if let Some(next) = find_next(&wall, &maze, &start) {
            maze.open(&next);
            maze.open(&wall);
            walls.append(&mut walls_for(&next, &maze));

            if let Progress::Delay(time) = progress {
                shared::print_part(&wall, &maze);
                shared::print_part(&next, &maze);
                io::stdout().flush().is_ok();
                thread::sleep(Duration::from_micros(time));
            }

            last = next.clone();
            frontier.push(next);
        };
    }

    maze.change(&start, Part::Start);
    maze.change(&last, Part::Finish);

    maze
}
