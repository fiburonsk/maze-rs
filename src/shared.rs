use super::maze::{print_maze, Maze, Part, Pos};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

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

pub fn draw_reset() {
    print!("{}[0;0f", 27 as char);
}

pub fn draw_at(pos: &Pos) {
    print!("{}[{};{}f", 27 as char, pos.y + 1, pos.x + 1);
}

pub fn draw_board(maze: &Maze, progress: &Progress) {
    if let Progress::Delay(time) = progress {
        draw_reset();
        print_maze(maze);
        io::stdout().flush().is_ok();
        thread::sleep(Duration::from_micros(*time));
    }
}

pub trait ChangeBoard {
    fn change(&mut self, pos: &Pos, to: Part);
    fn open(&mut self, pos: &Pos);
}

impl ChangeBoard for Maze {
    fn change(&mut self, pos: &Pos, to: Part) {
        self.board[pos.y][pos.x] = to;
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

pub fn pick_start(seed1: usize, seed2: usize, height: usize, width: usize) -> Pos {
    let x = usize::max(seed1 % (width - 1), 1);
    let y = usize::max(seed2 % (height - 1), 1);

    Pos { x, y }
}

pub fn print_part(pos: &Pos, m: &Maze) {
    draw_at(pos);
    print!("{}", m.at(pos));
}
