use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

use super::maze::{Blocks, Maze, Part, Pos};
use super::shared::{self, ChangeBoard, Direction, Movement, Progress};

fn pick_neighbor(pos: &Pos, m: &Maze, rng: &mut StdRng) -> Option<Direction> {
    let mut directions = shared::all_directions();
    directions.shuffle(rng);
    directions.into_iter().find(|dir| {
        m.go(pos, dir)
            .and_then(|w| m.go(&w, dir))
            .and_then(|c| if !m.is_open(&c) { Some(()) } else { None })
            .is_some()
    })
}

pub fn generate(seed: usize, height: usize, width: usize, progress: Progress) -> Maze {
    shared::clear_screen();
    let mut maze = Maze::new_empty(height, width);
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed as u64);
    let start = shared::pick_start(rng.gen::<usize>(), rng.gen::<usize>(), height, width);
    let mut visited: Blocks = vec![start.clone()];
    maze.open(&start);
    let mut last = start.clone();

    while let Some(current) = visited.pop() {
        if let Some(dir) = pick_neighbor(&current, &maze, &mut rng) {
            let wall = maze.go(&current, &dir).unwrap();
            maze.open(&wall);
            let next = maze.go(&wall, &dir).unwrap();
            maze.open(&next);
            last = next.clone();

            visited.push(current);
            visited.push(next);

            shared::draw_board(&maze, &progress);
        }
    }

    maze.change(&start, Part::Start);
    maze.change(&last, Part::Finish);

    maze
}
