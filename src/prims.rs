use rand::{rngs::StdRng, Rng, SeedableRng};
use std::thread;
use std::time::Duration;

use super::maze::{print_maze, Blocks, Maze, Part, Pos};
use super::shared::{self, ChangeBoard, Movement, Progress, Wall};

fn walls_for(pos: &Pos, m: &Maze) -> Blocks {
    shared::all_directions()
        .iter()
        .filter_map(|dir| m.go(pos, dir))
        .filter(|p| m.is_wall(p))
        .collect::<Blocks>()
}

fn open(m: &mut Maze, pos: &Pos) {
    m.change(pos, Part::Open);
}

fn find_cells(pos: &Pos, w: &Wall) -> (Pos, Pos) {
    match w {
        Wall::Horizontal => (pos.left(), pos.right()),
        Wall::Vertical => (pos.up(), pos.down()),
    }
}

fn check_wall(pos: &Pos, w: &Wall, m: &Maze) -> bool {
    let max_y = m.height_edge() - 1;
    let max_x = m.width_edge() - 1;

    match w {
        Wall::Horizontal => pos.x > 1 && pos.x < max_x,
        Wall::Vertical => pos.y > 1 && pos.y < max_y,
    }
}

fn wall_type(pos: &Pos, start: &Pos) -> Wall {
    if pos.x % 2 != start.x % 2 {
        Wall::Horizontal
    } else {
        Wall::Vertical
    }
}

fn check_cell(cell: (Pos, Pos), m: &Maze) -> Option<Pos> {
    if m.is_open(&cell.0) && !m.is_open(&cell.1) {
        Some(cell.1)
    } else if m.is_open(&cell.1) && !m.is_open(&cell.0) {
        Some(cell.0)
    } else {
        None
    }
}

pub fn generate(seed: usize, height: usize, width: usize, progress: Progress) -> Maze {
    shared::clear_screen();
    let mut maze = Maze::new_empty(height, width);
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed as u64);
    let start = shared::pick_start(rng.gen::<usize>(), rng.gen::<usize>(), height, width);
    let mut frontier: Blocks = vec![start.clone()];
    open(&mut maze, &start);
    let mut last = start.clone();
    let mut walls: Blocks = walls_for(&start, &maze);

    while !walls.is_empty() {
        let wall = {
            let index = rng.gen::<usize>() % walls.len();
            walls.remove(index)
        };

        let kind = wall_type(&wall, &start);

        if !check_wall(&wall, &kind, &maze) {
            continue;
        }

        let cells = find_cells(&wall, &kind);

        if let Some(next) = check_cell(cells, &maze) {
            open(&mut maze, &next);
            open(&mut maze, &wall);
            walls.append(&mut walls_for(&next, &maze));
            last = next.clone();
            frontier.push(next);

            if let Progress::Delay(time) = progress {
                shared::redraw();
                print_maze(&maze);
                thread::sleep(Duration::from_micros(time));
            }
        }
    }

    maze.change(&start, Part::Start);
    maze.change(&last, Part::Finish);

    maze
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let m = generate(10, 7, 7, Progress::Delay(100_000));
        print_maze(&m);
    }
}
