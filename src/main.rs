use std::collections::{
    HashSet,
};
use rand::{
    thread_rng,
    seq::SliceRandom,
};
use macroquad::prelude::{
    Color,
    BLACK,
    WHITE,
};
const WALL_COLOR: Color = BLACK;
const PATH_COLOR: Color = WHITE;
const MARGIN: f32 = 5.0;
const WIDTH: usize = 50;
const HEIGHT: usize = 50;

///Stores the direction of connections
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    N,
    W,
    E,
    S,
}

struct Edge {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Edge {
    fn dx(dir : Direction) -> i32{
        match dir{
            Direction::E => 1,
            Direction::W => -1,
            ____________ => 0,
        }
    }

    fn dy(dir : Direction) -> i32{
        match dir{
            Direction::N => -1,
            Direction::S => 1,
            ____________ => 0,
        }
    }

}

fn opposite(dir : Direction) -> Direction{
    match dir{
        Direction::N => Direction::S,
        Direction::S => Direction::N,
        Direction::E => Direction::W,
        Direction::W => Direction::E,
    }
}

impl From<Edge> for (usize, usize, Direction) {
    fn from(e: Edge) -> (usize, usize, Direction) {
        let Edge {x, y, dir} = e;
        (x, y, dir)
    }
}

#[macroquad::main("Maze")]
async fn main() {
    let grid = generate_maze(HEIGHT, WIDTH);
    loop {
        display_maze(&grid, HEIGHT, WIDTH).await;
    }

}

/// Generates the maze for later use
/// The arguments are maze height and width
fn generate_maze(height : usize, width : usize) -> Vec<Vec<HashSet<Direction>>>{
    // Create two dimensional array which can contain directions of connections
    let mut grid = vec![vec![HashSet::<Direction>::new(); width]; height];
    let mut sets = vec![vec![0; width]; height];
    let mut edges : Vec<Edge> = Vec::new();

    fill_sets(&mut sets, height, width);

    for y in  0..height {
        for x in 0..width {
            if  y > 0 {
                edges.push(Edge{x : x,y : y, dir : Direction::N})
            }
            if x > 0{
                edges.push(Edge{x : x, y : y, dir : Direction::W});
            }
        }
    }
    edges.shuffle(&mut thread_rng());

    for e in edges{
        let (x, y, dir) = <(usize, usize, Direction)>::from(e);
        let (nx, ny) = ((x as i32+Edge::dx(dir)) as usize, (y as i32+Edge::dy(dir)) as usize);

        let (set1, set2) = (sets[y][x], sets[ny][nx]);

        if set1 != set2{
            //display the maze here
            sets[y][x] = set2;
            sets.iter_mut().for_each(|vec2|{
                vec2.iter_mut().for_each(|i| if *i == set1 {*i = set2})
            });

            grid[y][x].insert(dir);
            grid[ny][nx].insert(opposite(dir));
        }

    }
    grid
}

fn fill_sets(sets : &mut Vec<Vec<usize>>, height : usize, width : usize){
    for i in 0..width{
        for j in 0..height{
            sets[j][i] = i + j*width;
        }
    }
}


async fn display_maze(grid : &Vec<Vec<HashSet<Direction>>>, height : usize, width : usize) {
    use macroquad::{
        prelude::*,
    };
    clear_background(WALL_COLOR);

    let cell_width = (screen_width()-MARGIN)/(width as f32);
    let cell_height = (screen_height()-MARGIN)/(height as f32);

    for i in 0..width {
        for j in 0..height {
            let x = (i as f32)*cell_width + MARGIN;
            let y = (j as f32)*cell_height + MARGIN;
            let w = cell_width-MARGIN;
            let h = cell_height-MARGIN;

            if i == 0 && j == 0 || i == width-1 && j == height-1{
                draw_rectangle(x, y, w, h, RED)
            } else {
                draw_rectangle(x, y, w, h, PATH_COLOR);
            }


            for d in &grid[j][i]{
                match d{
                    Direction::N =>
                        draw_rectangle(x, y - MARGIN, w, MARGIN, PATH_COLOR),
                    Direction::E =>
                        draw_rectangle(x + w, y, MARGIN, h, PATH_COLOR),
                    Direction::S =>
                        draw_rectangle(x, y + h, w, MARGIN, PATH_COLOR),
                    Direction::W =>
                        draw_rectangle(x - MARGIN, y, MARGIN, h, PATH_COLOR),
                }
            }
        }
    }



    next_frame().await
}
