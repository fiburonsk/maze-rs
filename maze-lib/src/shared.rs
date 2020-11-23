use super::maze::{Maze, Part, Pos};
use rand::{rngs::StdRng, Rng};

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn all_directions() -> Vec<Direction> {
    return vec![
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
}

pub fn opposite_dir(dir: &Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Progress {
    Delay(u64),
    None,
}

pub trait ChangeBoard {
    fn change(&mut self, pos: &Pos, to: Part);
    fn open(&mut self, pos: &Pos);
}

impl ChangeBoard for Maze {
    fn change(&mut self, pos: &Pos, to: Part) {
        let index = self.pos_to_index(pos);
        self.board[index] = to;
    }

    fn open(&mut self, pos: &Pos) {
        self.change(pos, Part::Open);
    }
}

pub trait Movement {
    fn go(&self, pos: &Pos, dir: &Direction) -> Option<Pos>;
}

impl Movement for Maze {
    fn go(&self, pos: &Pos, dir: &Direction) -> Option<Pos> {
        match dir {
            Direction::Up if pos.y > 0 => Some(pos.up()),
            Direction::Down if pos.y < self.height_edge() => Some(pos.down()),
            Direction::Right if pos.x < self.width_edge() => Some(pos.right()),
            Direction::Left if pos.x > 0 => Some(pos.left()),
            _ => None,
        }
    }
}

pub fn pick_end(rng: &mut StdRng, maze: &Maze) -> Pos {
    let height = maze.height_edge();
    let width = maze.width_edge();

    loop {
        let y = usize::max(rng.gen::<usize>() % height, 1);
        let p = Pos { x: width, y };

        if maze.is_open(&p) || maze.is_open(&Pos { x: width - 1, y }) {
            return p;
        }
    }
}
