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

#[derive(Debug, PartialEq)]
pub enum Progress {
    Delay(u64),
    None,
}

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}
