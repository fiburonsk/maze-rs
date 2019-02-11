#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum Progress {
    Delay(u64),
    None,
}

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}
