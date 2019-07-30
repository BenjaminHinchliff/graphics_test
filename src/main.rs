// use piston::window::WindowSettings;
// use piston::event_loop::*;
// use piston::input::*;
// use glutin_window::GlutinWindow as Window;
// use opengl_graphics::{ GlGraphics, OpenGL };
use disjoint_sets::UnionFind;
use rand::{ thread_rng, seq::SliceRandom };

#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug)]
struct Wall {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Wall {
    fn new(x: usize, y: usize, direction: Direction) -> Wall {
        Wall {
            x,
            y,
            direction,
        }
    }
}

fn printGrid(grid: &Vec<Vec<isize>>) {
    for line in grid {
        println!("{:?}", line);
    }
}

fn main() {
    let grid_width = 5;
    let grid_height = 5;

    let mut grid = vec![vec![0; grid_width]; grid_height];
    let mut sets = UnionFind::<u8>::new(grid_height * grid_width);

    let mut walls = Vec::new();
    for y in 1..grid_height {
        for x in 1..grid_width {
            walls.push(Wall::new(x, y, Direction::N));
            walls.push(Wall::new(x, y, Direction::W))
        }
    }

    walls.shuffle(&mut thread_rng());

    println!("{:?}", walls);

    printGrid(&grid)
}