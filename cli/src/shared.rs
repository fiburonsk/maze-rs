use std::fmt;

use maze::maze::{Maze, Part, Pos};
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

pub fn print_visited() {
    print!("\x1b[0;33m+\x1b[0m");
}

pub fn print_part(pos: &Pos, m: &Maze) {
    draw_at(pos);
    print!("{}", CliPart::new(m.at(pos)));
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

pub fn print_maze(maze: &Maze) {
    let width = maze.width;
    for (index, part) in maze.board.iter().enumerate() {
        if index != 0 && index % width == 0 {
            println!()
        }
        print!("{}", CliPart::new(*part));
    }

    println!()
}

pub fn draw_board(maze: &Maze, progress: &Progress) {
    if let Progress::Delay(time) = progress {
        draw_reset();
        print_maze(maze);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_micros(*time));
    }
}

pub struct CliPart(Part);

use std::ops::Deref;

impl Deref for CliPart {
    type Target = Part;

    fn deref(&self) -> &Part {
        &self.0
    }
}

impl CliPart {
    pub fn new(part: Part) -> CliPart {
        CliPart(part)
    }
}

impl fmt::Display for CliPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match &self.0 {
            Part::Wall => "\x1b[90m\u{2588}\x1b[0m",
            Part::Open => " ",
            Part::Start => "\x1b[1;33ms\x1b[0m",
            Part::Finish => "\x1b[1;33mf\x1b[0m",
        };

        write!(f, "{}", c)
    }
}
