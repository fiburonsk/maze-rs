use image;
use maze_lib::maze::{Maze, Part, Pos};

pub fn save(maze: &Maze, solution: &[Pos], name: &str) {
    let imgx = maze.width() as u32;
    let imgy = maze.height() as u32;
    let wall: image::Rgb<u8> = image::Rgb([128, 128, 128]);
    let path: image::Rgb<u8> = image::Rgb([48, 48, 48]);
    let start: image::Rgb<u8> = image::Rgb([0, 255, 0]);
    let finish: image::Rgb<u8> = image::Rgb([255, 0, 0]);
    let visit: image::Rgb<u8> = image::Rgb([225, 200, 128]);

    let mut buf = image::ImageBuffer::new(imgx, imgy);

    (0..imgy).for_each(|y| {
        (0..imgx).for_each(|x| {
            let pos = Pos {
                x: x as usize,
                y: y as usize,
            };
            let p = buf.get_pixel_mut(x as u32, y as u32);
            *p = match &maze.at(&pos) {
                Part::Wall => wall,
                Part::Start => start,
                Part::Finish => finish,
                Part::Open => path,
            };
        });
    });

    solution.iter().for_each(|pos| {
        let p = buf.get_pixel_mut(pos.x as u32, pos.y as u32);
        if let Part::Open = &maze.at(pos) {
            *p = visit;
        }
    });

    buf.save(name).unwrap();
}
