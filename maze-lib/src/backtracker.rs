use super::{
    maze::{Maze, Pos},
    shared::{self, Direction, Movement},
};
use rand::{rngs::StdRng, seq::SliceRandom};

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

pub fn step(maze: &Maze, rng: &mut StdRng, pos: &Pos) -> Option<(Pos, Pos)> {
    if let Some(dir) = pick_neighbor(pos, maze, rng) {
        let wall = maze.go(pos, &dir).expect("Should go to direction");
        let next = maze.go(&wall, &dir).expect("Should continue in direction");

        Some((wall, next))
    } else {
        None
    }
}
