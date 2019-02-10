use std::fmt;

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
            Part::Wall => '\u{2588}',
            Part::Open => ' ',
            Part::Start => 's',
            Part::Finish => 'f',
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

pub struct Maze {
    pub board: Vec<Vec<Part>>,
}

impl Maze {
    pub fn at(&self, pos: &Pos) -> Part {
        self.board[pos.y][pos.x]
    }

    pub fn is_wall(&self, pos: &Pos) -> bool {
        self.at(pos) == Part::Wall
    }

    pub fn is_finished(&self, pos: &Pos) -> bool {
        self.at(pos) == Part::Finish
    }

    pub fn is_start(&self, pos: &Pos) -> bool {
        self.at(pos) == Part::Start
    }

    pub fn start_at(&self) -> Option<Pos> {
        self.board.iter().enumerate().find_map(|(row, cols)| {
            if let Some(found) = cols.iter().position(|col| *col == Part::Start) {
                return Some(Pos { x: found, y: row });
            }

            None
        })
    }

    pub fn finish_at(&self) -> Option<Pos> {
        self.board.iter().enumerate().find_map(|(row, cols)| {
            if let Some(found) = cols.iter().position(|col| *col == Part::Finish) {
                return Some(Pos { x: found, y: row });
            }

            None
        })
    }
}

pub fn make() -> Vec<Vec<Part>> {
    let b = Part::Wall;
    let o = Part::Open;
    let s = Part::Start;
    let f = Part::Finish;

    vec![
        vec![b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b],
        vec![b, s, o, o, o, o, o, o, o, o, o, o, b, b, b, b, o, b],
        vec![b, b, b, o, b, b, b, b, b, b, b, o, o, o, o, o, o, b],
        vec![b, o, b, o, b, o, b, b, o, o, o, b, b, b, b, o, b, b],
        vec![b, o, b, o, b, o, o, o, o, b, o, o, o, o, o, o, o, b],
        vec![b, o, o, o, b, o, b, b, b, b, b, b, b, b, b, b, b, b],
        vec![b, o, b, b, b, o, b, b, o, b, o, o, o, o, o, o, f, b],
        vec![b, o, b, o, o, o, o, o, o, b, o, b, b, b, b, b, o, b],
        vec![b, o, b, b, b, b, b, b, o, b, o, o, o, b, o, o, o, b],
        vec![b, o, b, o, o, o, o, b, o, b, b, b, b, b, b, b, o, b],
        vec![b, o, b, b, b, o, b, b, o, o, o, o, o, b, o, b, o, b],
        vec![b, o, o, o, b, o, b, b, o, b, b, b, o, o, o, o, o, b],
        vec![b, o, b, b, b, o, b, b, o, b, o, b, b, b, o, b, b, b],
        vec![b, o, o, o, o, o, o, b, o, o, o, o, o, o, o, o, o, b],
        vec![b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b, b],
    ]
}
