use maze_lib::{
    maze::{Maze, Part, Pos},
    shared::Progress,
};

use std::fmt;
use std::io::{self, Write};
use std::ops::Deref;
use std::thread;
use std::time::Duration;

pub struct CliPart(Part);

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
