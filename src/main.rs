// use piston::window::WindowSettings;
// use piston::event_loop::*;
// use piston::input::*;
// use glutin_window::GlutinWindow as Window;
// use opengl_graphics::{ GlGraphics, OpenGL };
use disjoint_sets::UnionFind;

// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

fn main() {
    let grid_width = 5;
    let grid_height = 5;

    let mut grid = vec![vec![UnionFind::<u8>::new(1); grid_width]; grid_height];

    //grid[0][0].union(grid[0][1]);

    println!("{:?}", grid)
}