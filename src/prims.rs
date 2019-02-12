use rand::{rngs::StdRng, Rng, SeedableRng};
use std::thread;
use std::time::Duration;

use super::maze::{print_maze, Maze, Part, Pos};
use super::shared::{self, Direction, Progress};

const MIN: usize = 2;
const MAX: usize = 3;

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

fn neighbors(pos: &Pos, m: &Maze) -> Blocks {
    let mut n = vec![];
    let max_y = m.height() - MAX;
    let max_x = m.width() - MAX;

    let check = |p: &Pos| -> bool { m.at(p) == Part::Wall || m.at(p) == Part::Frontier };

    if pos.y > MIN {
        let up = pos.up().up();
        if check(&up) {
            n.push(up);
        }
    }

    if pos.x < max_x {
        let right = pos.right().right();
        if check(&right) {
            n.push(right);
        }
    }

    if pos.y < max_y {
        let down = pos.down().down();

        if check(&down) {
            n.push(down);
        }
    }

    if pos.x > MIN {
        let left = pos.left().left();

        if check(&left) {
            n.push(left);
        }
    }

    n
}

fn mark(pos: &Pos, m: &mut Maze, frontier: &mut Blocks) {
    open(m, pos);
    let max_y = m.height() - MAX;
    let max_x = m.width() - MAX;

    if pos.y > MIN {
        add_frontier(&(pos.up().up()), m, frontier);
    }

    if pos.x < max_x {
        add_frontier(&(pos.right().right()), m, frontier);
    }

    if pos.y < max_y {
        add_frontier(&(pos.down().down()), m, frontier);
    }

    if pos.x > MIN {
        add_frontier(&(pos.left().left()), m, frontier);
    }
}

fn open(m: &mut Maze, pos: &Pos) {
    m.change(pos, Part::Open);
}

fn add_frontier(pos: &Pos, m: &mut Maze, frontier: &mut Blocks) {
    if Part::Wall == m.at(pos) {
        frontier.push(pos.clone());
        m.change(pos, Part::Frontier);
    }
}

fn find_middle(current: &Pos, next: &Pos) -> Pos {
    if current.x == next.x {
        if current.y > next.y {
            current.up()
        } else {
            current.down()
        }
    } else if current.x > next.x {
        current.left()
    } else {
        current.right()
    }
}

fn find_opposite(current: &Pos, next: &Pos, m: &Maze) -> Option<Pos> {
    let max_y = m.height() - MAX;
    let max_x = m.width() - MAX;

    if current.x == next.x {
        if current.y > next.y && current.y < max_y {
            Some(current.down().down())
        } else if current.y < next.y && current.y > MIN {
            Some(current.up().up())
        } else {
            None
        }
    } else if current.x > next.x && current.x < max_x {
        Some(current.right().right())
    } else if current.x < next.x && current.x > MIN {
        Some(current.left().left())
    } else {
        None
    }
}

pub fn generate(seed: usize, height: usize, width: usize, progress: Progress) -> Maze {
    let board = (0..height)
        .map(|_y| (0..width).map(|_x| Part::Wall).collect::<Row>())
        .collect::<Vec<Row>>();

    let mut maze = Maze { board: board };
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed as u64);
    let start = Pos { x: 1, y: 1 };
    let mut frontier: Blocks = vec![start.clone()];
    let mut last = start.clone();

    while !frontier.is_empty() {
        let current = {
            let index = rng.gen::<usize>() % frontier.len();
            frontier.remove(index)
        };

        last = current.clone();

        let mut n = neighbors(&current, &maze);

        mark(&current, &mut maze, &mut frontier);

        if progress != Progress::None {
            shared::clear_screen();
            print_maze(&maze);
        }

        if !n.is_empty() {
            let next = {
                let index = rng.gen::<usize>() % n.len();
                n.remove(index)
            };

            let middle = find_middle(&current, &next);
            if let Some(opposite) = find_opposite(&current, &next, &maze) {
                if maze.at(&opposite) == Part::Open {
                    let wall = find_middle(&current, &opposite);
                    open(&mut maze, &wall);
                }
            }

            open(&mut maze, &middle);

            if progress != Progress::None {
                shared::clear_screen();
                print_maze(&maze);
            }
        }

        if let Progress::Delay(time) = progress {
            thread::sleep(Duration::from_micros(time));
        }
    }

    maze.change(&start, Part::Start);
    maze.change(&last, Part::Finish);

    maze
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let m = generate(10, 7, 7, Progress::Delay(100_000));
        print_maze(&m);
    }
}
