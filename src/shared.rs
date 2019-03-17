use super::maze::{print_maze, Maze, Part, Pos};
use rand::{rngs::StdRng, Rng};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

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

pub fn print_visited() {
    print!("\x1b[0;33m+\x1b[0m");
}

pub fn print_part(pos: &Pos, m: &Maze) {
    draw_at(pos);
    print!("{}", m.at(pos));
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
