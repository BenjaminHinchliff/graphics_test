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

impl Direction {
    fn opposite(self) -> Direction {
        use Direction::*;
        match self {
            N => S,
            E => W,
            S => N,
            W => E,
        }
    }

    fn apply(&self, x: usize, y: usize) -> (usize, usize) {
        use Direction::*;
        match self {
            N => (x, y + 1),
            E => (x + 1, y),
            S => (x, y - 1),
            W => (x - 1, y),
        }
    }
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

fn print_grid(grid: &Vec<Vec<isize>>) {
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

    while !walls.is_empty() {
        let Wall{ x, y, direction } = walls.pop().unwrap();
        let (nx, ny) = direction.apply(x, y);

        let (set1, set2) = (sets.find((x * y) as u8), sets.find((nx * ny) as u8));

        


        println!("X: {} Y: {} NX: {} NY: {}", x, y, nx, ny);
    }

    println!("{:?}", walls);

    print_grid(&grid)
}