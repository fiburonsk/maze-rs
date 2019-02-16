use super::maze::{Maze, Part, Pos};

#[derive(Debug, PartialEq)]
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

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}

pub fn redraw() {
    print!("{}[0;0f", 27 as char);
}

pub enum Wall {
    Horizontal,
    Vertical,
}

pub fn pick_start(seed1: usize, seed2: usize, height: usize, width: usize) -> Pos {
    let x = usize::max(seed1 % (width - 1), 1);
    let y = usize::max(seed2 % (height - 1), 1);

    Pos { x, y }
}

pub trait ChangeBoard {
    fn change(&mut self, pos: &Pos, to: Part);
}

impl ChangeBoard for Maze {
    fn change(&mut self, pos: &Pos, to: Part) {
        self.board[pos.y][pos.x] = to;
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
