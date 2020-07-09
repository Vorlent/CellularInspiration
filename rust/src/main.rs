use rayon::prelude::*;
use ndarray::{s, Array2, ArrayView2, Zip};

fn main() {
    let mut grid = Array2::<i32>::zeros((10, 10));
    let mut next_grid = Array2::<i32>::zeros((10, 10));

    let inner_grid = grid.slice(s![1..-1, 1..-1]);
    let mut inner_next_grid = next_grid.slice_mut(s![1..-1, 1..-1]);
    Zip::indexed(inner_grid)
        .and(&mut inner_next_grid)
        .par_apply(|(x,y), v, nv| {
        let outer_x = x + 1;
        let outer_y = y + 1;
        println!("cell {},{} has value {}", outer_x, outer_y, v);
        let neighbours = grid.slice(s![(outer_x-1)..=(outer_x+1), (outer_y-1)..=(outer_y+1)]);
        let living_neighbours = neighbours.indexed_iter().fold(0, |acc, ((n_x,n_y), n_v)| {
            if n_x != 1 && n_y != 1 && grid[[n_x,n_y]] != 0 {
                acc + 1
            } else {
                acc
            }
        });
        if living_neighbours < 2 || living_neighbours > 3 {
            println!("cell dies");
            *nv = 0;
        }
        if living_neighbours == 3 {
            println!("cell lives or reproduces");
            *nv = 1;
        }
    });
}
