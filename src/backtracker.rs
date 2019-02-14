use rand::{rngs::StdRng, Rng, SeedableRng};
use std::thread;
use std::time::Duration;

use super::maze::{print_maze, Maze, Part, Pos};
use super::shared::{self, Progress};

type Row = Vec<Part>;
type Blocks = Vec<Pos>;

trait ChangeBoard {
    fn change(&mut self, pos: &Pos, to: Part);
}

impl ChangeBoard for Maze {
    fn change(&mut self, pos: &Pos, to: Part) {
        self.board[pos.y][pos.x] = to;
    }
}

fn open(m: &mut Maze, pos: &Pos) {
    m.change(pos, Part::Open);
}

fn find_neighbors(pos: &Pos, m: &Maze) -> Blocks {
    let max_x = m.width() - 3;
    let max_y = m.height() - 3;
    let min = 2;
    let mut n = vec![];

    if pos.x > min {
        let next = pos.left().left();
        if m.is_open(&next) {
            n.push(next);
        }
    }

    if pos.y > min {
        let next = pos.up().up();
        if m.is_open(&next) {
            n.push(next);
        }
    }

    if pos.x < max_x {
        let next = pos.right().right();
        if m.is_open(&next) {
            n.push(next);
        }
    }

    if pos.y < max_y {
        let next = pos.down().down();
        if m.is_open(&next) {
            n.push(next);
        }
    }

    n
}

fn find_wall(current: &Pos, next: &Pos) -> Pos {
    if current.x == next.x && current.y > next.y {
        current.up()
    } else if current.x == next.x {
        current.down()
    } else if current.y == next.y && current.x > next.x {
        current.left()
    } else {
        current.right()
    }
}

pub fn generate(seed: usize, height: usize, width: usize, progress: Progress) -> Maze {
    shared::clear_screen();
    let board = (0..height)
        .map(|_y| (0..width).map(|_x| Part::Wall).collect::<Row>())
        .collect::<Vec<Row>>();

    let mut maze = Maze { board: board };
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed as u64);
    let start = shared::pick_start(
        rng.gen::<usize>(),
        rng.gen::<usize>(),
        maze.height(),
        maze.width(),
    );
    let mut visited: Blocks = vec![start.clone()];
    let mut last = start.clone();

    while let Some(current) = visited.pop() {
        open(&mut maze, &current);

        let mut neighbors = find_neighbors(&current, &maze);

        if !neighbors.is_empty() {
            let next = {
                let index = rng.gen::<usize>() % neighbors.len();
                neighbors.remove(index)
            };

            open(&mut maze, &next);
            let wall = find_wall(&current, &next);
            open(&mut maze, &wall);

            last = next.clone();

            visited.push(current);
            visited.push(next);
        }

        if let Progress::Delay(time) = progress {
            shared::redraw();
            print_maze(&maze);
            thread::sleep(Duration::from_micros(time));
        }
    }

    maze.change(&start, Part::Start);
    maze.change(&last, Part::Finish);

    maze
}
