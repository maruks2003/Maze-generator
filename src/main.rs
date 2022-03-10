use std::collections::{
    HashSet,
};
use rand::{
    thread_rng,
    seq::SliceRandom,
};

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
    let width = 100;
    let height = 100;
    let grid = generate_maze(height, width);
    loop {
        display_maze(&grid, height, width).await;
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
    clear_background(BLACK);

    let margin : f32 = 10.0;
    let cell_width = (macroquad::window::screen_width()-margin)/(width as f32);
    let cell_height = (macroquad::window::screen_height()-margin)/(height as f32);

    for i in 0..width {
        for j in 0..height {
            let x = (i as f32)*cell_width + margin;
            let y = (j as f32)*cell_height + margin;
            let w = cell_width-margin;
            let h = cell_height-margin;
            draw_rectangle(x, y, w, h, RED);

            for d in &grid[j][i]{
                match d{
                    Direction::N =>
                        draw_rectangle(x, y - margin, w, margin, RED),
                    Direction::E =>
                        draw_rectangle(x + w, y, margin, h, RED),
                    Direction::S =>
                        draw_rectangle(x, y + h, w, margin, RED),
                    Direction::W =>
                        draw_rectangle(x - margin, y, margin, h, RED),
                }
            }
        }
    }



    next_frame().await
}
