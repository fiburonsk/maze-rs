pub type Blocks = Vec<Pos>;

#[derive(Copy, Clone, PartialEq)]
pub enum Part {
    Wall,
    Open,
    Start,
    Finish,
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
    pub height: usize,
    pub width: usize,
    pub board: Vec<Part>,
}

impl Maze {
    pub fn new_empty(height: usize, width: usize) -> Self {
        let length = height * width;
        Maze {
            height,
            width,
            board: std::vec::from_elem(Part::Wall, length),
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn height_edge(&self) -> usize {
        self.height() - 1
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn width_edge(&self) -> usize {
        self.width() - 1
    }

    pub fn at(&self, pos: &Pos) -> Part {
        self.board[self.pos_to_index(pos)]
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

    pub fn to_pos(&self, index: usize) -> Pos {
        let x = index % self.width;
        let y = index / self.width;
        return Pos { x, y };
    }

    pub fn pos_to_index(&self, pos: &Pos) -> usize {
        pos.y * self.width + pos.x
    }

    pub fn start_at(&self) -> Option<Pos> {
        self.board.iter().enumerate().find_map(|(index, part)| {
            if *part == Part::Start {
                return Some(self.to_pos(index));
            }

            None
        })
    }
}
