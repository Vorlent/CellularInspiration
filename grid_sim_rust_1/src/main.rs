use rayon::prelude::*;
use ndarray::{s, Array2, ArrayView2, Zip};
use std::cmp;

#[derive(Copy, Clone, Debug)]
enum BlockType {
    None,
    Chest,
    Hopper
}

#[derive(Copy, Clone, Debug)]
enum ItemType {
    None,
    Apple,
    Carrot
}

#[derive(Copy, Clone)]
struct Block {
    block_type: BlockType,
    item_type: ItemType,
    direction_x: i32,
    direction_y: i32
}

impl Block {
    fn update_neighbour(&self, x: i32, y: i32, nx: i32, ny: i32, cv: &Block, nxtv: &mut Block) -> () {
        match self.block_type {
            // you are a hopper x, y
            BlockType::Hopper => {
                match cv.block_type {
                    // your neighbour cv is a chest nx, ny
                    BlockType::Chest => {
                        match self.item_type {
                            // if the hopper is empty
                            ItemType::None => {
                                // and the hopper is pointing away from the chest
                                // then empty the chest
                                if self.direction_x != nx || self.direction_y != ny {
                                    println!("Transfer from Chest ({},{}) to Hopper ({},{})!", nx, ny, x, y);
                                    println!("Chest ({},{}) deletes its item", nx, ny);
                                    nxtv.item_type = ItemType::None;
                                }
                            },
                            // if the hopper is full
                            _ => {
                                // and the hopper is at this chest
                                // then fill the chest with the item from the hopper
                                if self.direction_x == nx && self.direction_y == ny {
                                    println!("Transfer to Chest ({},{}) from Hopper ({},{})!", nx, ny, x, y);
                                    println!("Chest ({},{}) adds item ({:?}) from Hopper ({},{})", nx, ny, self.item_type, x, y);
                                    nxtv.item_type = self.item_type;
                                }
                            }
                        }
                    }
                    _ => ()
                }
            },
            // you are a chest x, y
            BlockType::Chest => {
                match cv.block_type {
                    // your neighbour cv is a hopper nx, ny
                    BlockType::Hopper => {
                        match self.item_type {
                            // if the chest is empty
                            ItemType::None => {
                                // and the hopper points at this chest
                                if cv.direction_x == x && cv.direction_y == y {
                                    match cv.item_type {
                                        ItemType::None => {},
                                        // and the hopper is not empty
                                        _ => {
                                            // then empty the hopper
                                            println!("Transfer to Chest ({},{}) from Hopper ({},{})!", x, y, nx, ny);
                                            println!("Hopper ({},{}) deletes its item", nx, ny);
                                            nxtv.item_type = ItemType::None;
                                        }
                                    }
                                }
                            },
                            _ => {
                                // if the chest is full
                                // and the hopper is pointing away
                                if cv.direction_x != x || cv.direction_y != y {
                                    match cv.item_type {
                                        // and the hopper is empty
                                        ItemType::None => {
                                            // then fill the hopper with the item from the chest
                                            println!("Transfer from Chest ({},{}) to Hopper ({},{})!", x, y, nx, ny);
                                            println!("Hopper ({},{}) adds item ({:?}) from Chest ({},{})", nx, ny, self.item_type, x, y);
                                            nxtv.item_type = self.item_type;
                                        },
                                        _ => {}
                                    }
                                }
                            }
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }
}

const N: usize = 2000;

// bounded grid coordinate
fn bgc(coord: i32) -> i32 {
    cmp::min((N as i32)-1, cmp::max(0, coord))
}

fn main() {
    let empty_block = Block { block_type: BlockType::None, item_type: ItemType::None, direction_x: 0, direction_y: 0 };

    let mut grid = Array2::<Block>::from_elem((N, N), empty_block);

    grid[[3,3]] = Block { block_type: BlockType::Chest, item_type: ItemType::Apple, direction_x: 0, direction_y: 0 };
    grid[[3,4]] = Block { block_type: BlockType::Hopper, item_type: ItemType::None, direction_x: 3, direction_y: 5 };
    grid[[3,5]] = Block { block_type: BlockType::Chest, item_type: ItemType::None, direction_x: 0, direction_y: 0 };

    let mut next_grid = Array2::<Block>::from_elem((N, N), empty_block);
    show_grid(&grid);

    step(&grid, &mut next_grid);
    show_grid(&next_grid);

    println!("Step 2");
    step(&next_grid, &mut grid);
    show_grid(&grid);
}

fn show_grid(grid: &Array2::<Block>) -> () {
    println!();
    Zip::indexed(grid)
        .apply(|(x,y), v| {
            match v.block_type {
                BlockType::Chest => {
                    println!("Chest ({},{}) WITH {:?}", x, y, v.item_type)
                },
                BlockType::Hopper => {
                    println!("Hopper ({},{}) WITH {:?}", x, y, v.item_type)
                },
                _ => ()
            }
    });
    println!();
}

fn step(grid: &Array2::<Block>, next_grid: &mut Array2::<Block>) -> () {
    Zip::indexed(next_grid)
        .par_apply(|(x,y), mut nxtv| {
            let cv = grid[[x,y]];
            // copy all current data to the new block
            nxtv.block_type = cv.block_type;
            nxtv.item_type = cv.item_type;
            nxtv.direction_x = cv.direction_x;
            nxtv.direction_y = cv.direction_y;
            let neighbours = grid.slice(s![bgc((x as i32)-1)..=bgc((x as i32)+1), bgc((y as i32)-1)..=bgc((y as i32)+1)]);
            Zip::indexed(neighbours).apply(|(nx,ny), nv| {
                nv.update_neighbour((nx as i32) + (x as i32)-1, (ny as i32) + (y as i32)-1, x as i32, y as i32, &cv, &mut nxtv);
            })
    });
}
