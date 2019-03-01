use super::maze::{Blocks, Maze, Pos};
use super::shared::{self, Direction, Movement, Progress};
use std::io::{self, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

enum Run {
    Start(Pos),
    Solve(Branch),
    Solution(Blocks),
    None,
}

struct Branch {
    at: Pos,
    dir: Direction,
    path: Vec<Pos>,
}

pub fn solve(maze: &Maze, show_solve: &Progress) -> Option<Blocks> {
    let new_maze = maze.clone();
    let progress = show_solve.clone();

    let arc_maze = Arc::new(new_maze);
    let arc_progress = Arc::new(Mutex::new(progress));

    run(arc_maze.clone(), arc_progress)
}

fn run(maze: Arc<Maze>, progress: Arc<Mutex<Progress>>) -> Option<Blocks> {
    {
        let p = progress.lock().unwrap();
        shared::clear_screen();
        shared::draw_board(&maze, &p);
    }
    println!("Solve the maze!");

    let start = match maze.start_at() {
        Some(pos) => pos,
        None => return None,
    };

    let (wtx, wrx) = mpsc::channel();
    let (mtx, mrx) = mpsc::channel();

    let wrk_sender = wtx.clone();
    let wrk_progress = progress.clone();
    let wrk_maze = maze.clone();
    let mwrk_sender = mtx.clone();
    let threads: Arc<Mutex<Vec<Option<thread::JoinHandle<_>>>>> = Arc::new(Mutex::new(vec![]));
    let wrk_threads = threads.clone();
    let work = thread::spawn(move || {
        for recv in wrx.iter() {
            match recv {
                Run::Start(pos) => {
                    let thr_sender = wrk_sender.clone();
                    let thr_maze = wrk_maze.clone();

                    thread::spawn(move || {
                        begin(pos, thr_sender, &thr_maze);
                    });
                }
                Run::Solve(branch) => {
                    let thr_sender = wrk_sender.clone();
                    let thr_maze = wrk_maze.clone();
                    let thr_progress = wrk_progress.clone();

                    let j = thread::spawn(move || {
                        solver(branch, thr_sender, &thr_maze, thr_progress);
                    });

                    {
                        let mut v = wrk_threads.lock().unwrap();
                        v.push(Some(j));
                    }
                }
                Run::Solution(path) => {
                    mwrk_sender.send(path).unwrap();
                    break;
                }
                _ => break,
            }
        }
    });

    wtx.send(Run::Start(start.clone())).unwrap();
    drop(wtx);

    work.join().is_ok();

    // Let the threads finish up before moving on.
    let mut t = threads.lock().unwrap();
    t.iter_mut().for_each(|some_handle| {
        if let Some(handle) = some_handle.take() {
            handle.join().is_ok();
        }
    });

    if let Ok(path) = mrx.recv() {
        Some(path)
    } else {
        None
    }
}

fn begin(start: Pos, tx: mpsc::Sender<Run>, maze: &Maze) {
    shared::all_directions()
        .into_iter()
        .filter(|d| {
            maze.go(&start, &d)
                .and_then(|p| if !maze.is_wall(&p) { Some(()) } else { None })
                .is_some()
        })
        .for_each(|d| {
            tx.send(Run::Solve(Branch {
                at: start.clone(),
                dir: d,
                path: vec![start.clone()],
            }))
            .unwrap()
        });
}

fn solver(branch: Branch, tx: mpsc::Sender<Run>, maze: &Maze, progress: Arc<Mutex<Progress>>) {
    let mut at = branch.at.clone();
    let mut dir = branch.dir;
    let path = branch.path.clone();

    let prog = {
        let p = progress.lock().unwrap();
        p.clone()
    };

    let mut visited: Blocks = vec![];

    while let Some(next) = maze.go(&at, &dir) {
        at = next;
        visited.push(at.clone());
        if let Progress::Delay(time) = prog {
            {
                let _p = progress.lock().unwrap();
                shared::draw_at(&at);
                shared::print_visited();
                io::stdout().flush().is_ok();
            }
            thread::sleep(Duration::from_micros(time));
        }

        if maze.is_finished(&at) {
            let mut new_path = path.clone();
            new_path.append(&mut visited.clone());
            tx.send(Run::Solution(new_path)).is_ok();
        }

        let mut moves: Vec<Direction> = shared::all_directions()
            .into_iter()
            .filter(|d| {
                *d != shared::opposite_dir(&dir)
                    && maze
                        .go(&at, &d)
                        .and_then(|p| if !maze.is_wall(&p) { Some(()) } else { None })
                        .is_some()
            })
            .collect();

        if moves.is_empty() {
            break;
        }

        dir = moves.pop().unwrap();

        moves.into_iter().for_each(|d| {
            let mut new_path = path.clone();
            new_path.append(&mut visited.clone());

            tx.send(Run::Solve(Branch {
                at: at.clone(),
                dir: d,
                path: new_path,
            }))
            .is_ok();
        });
    }
}