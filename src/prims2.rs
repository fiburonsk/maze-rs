use rand::{rngs::StdRng, Rng, SeedableRng};

use super::maze::{Blocks, Maze, Part, Pos};
use super::shared::{self, ChangeBoard, Direction, Movement, Progress};
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

fn find_direction(pos1: &Pos, pos2: &Pos) -> Direction {
    if pos1.x == pos2.x && pos1.y > pos2.y {
        Direction::Up
    } else if pos1.x == pos2.x {
        Direction::Down
    } else if pos1.y == pos2.y && pos1.x > pos2.x {
        Direction::Left
    } else {
        Direction::Right
    }
}

fn rand_direction(rng: &mut StdRng, dir: &Direction) -> Direction {
    let op = shared::opposite_dir(&dir);

    let dirs: Vec<Direction> = shared::all_directions()
        .into_iter()
        .map(|d| if d == op { dir.clone() } else { d })
        .collect();

    let pick = rng.gen::<usize>() % dirs.len();
    dirs[pick].clone()
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
    if let Progress::Delay(_) = progress {
        shared::clear_screen();
    }
    let mut maze = Maze::new_empty(height, width);
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed as u64);
    let start = Pos { x: 1, y: 1 };
    maze.open(&start);
    let mut walls: Blocks = walls_for(&start, &maze);
    shared::draw_board(&maze, &progress);

    while !walls.is_empty() {
        let mut wall = {
            let index = rng.gen::<usize>() % walls.len();
            walls.remove(index)
        };

        (0..)
            .take_while(|_i| {
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

                    let dir = find_direction(&wall, &next);

                    if let Some(goto) = maze.go(&next, &rand_direction(&mut rng, &dir)) {
                        wall = goto;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .for_each(drop);
    }

    maze.change(&start, Part::Start);
    maze.change(&shared::pick_end(&mut rng, &maze), Part::Finish);

    maze
}
