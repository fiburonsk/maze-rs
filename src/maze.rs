use std::fmt;

pub type Row = Vec<Part>;
pub type Blocks = Vec<Pos>;

#[derive(Copy, Clone, PartialEq)]
pub enum Part {
    Wall,
    Open,
    Start,
    Finish,
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match &self {
            Part::Wall => "\x1b[90m\u{2588}\x1b[0m",
            Part::Open => " ",
            Part::Start => "\x1b[1;33ms\x1b[0m",
            Part::Finish => "\x1b[1;33mf\x1b[0m",
        };

        write!(f, "{}", c)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn up(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn down(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn left(&self) -> Pos {
        Pos {
            x: self.x - 1,
            y: self.y,
        }
    }
    pub fn right(&self) -> Pos {
        Pos {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[derive(Clone)]
pub struct Maze {
    pub board: Vec<Vec<Part>>,
}

impl Maze {
    pub fn new_empty(height: usize, width: usize) -> Self {
        Maze {
            board: (0..height)
                .map(|_y| (0..width).map(|_x| Part::Wall).collect::<Row>())
                .collect::<Vec<Row>>(),
        }
    }

    pub fn height(&self) -> usize {
        self.board.len()
    }

    pub fn height_edge(&self) -> usize {
        self.height() - 1
    }

    pub fn width(&self) -> usize {
        if self.board.is_empty() {
            0
        } else {
            self.board[0].len()
        }
    }

    pub fn width_edge(&self) -> usize {
        self.width() - 1
    }

    pub fn at(&self, pos: &Pos) -> Part {
        self.board[pos.y][pos.x]
    }

    pub fn is_open(&self, pos: &Pos) -> bool {
        self.at(pos) == Part::Open
    }

    pub fn is_wall(&self, pos: &Pos) -> bool {
        self.at(pos) == Part::Wall
    }

    pub fn is_finished(&self, pos: &Pos) -> bool {
        self.at(pos) == Part::Finish
    }

    pub fn start_at(&self) -> Option<Pos> {
        self.board.iter().enumerate().find_map(|(row, cols)| {
            if let Some(found) = cols.iter().position(|col| *col == Part::Start) {
                return Some(Pos { x: found, y: row });
            }

            None
        })
    }
}

pub fn print_maze(maze: &Maze) {
    for row in maze.board.iter() {
        for col in row.iter() {
            print!("{}", &col);
        }
        println!();
    }
}
