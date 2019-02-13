use super::maze::Pos;

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
