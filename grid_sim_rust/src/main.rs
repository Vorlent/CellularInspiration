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
            BlockType::Hopper => {
                match cv.block_type {
                    BlockType::Chest => {
                        match self.item_type {
                            ItemType::None => {
                                if(self.direction_x == nx && self.direction_y == ny) {
                                    println!("1 Transfer from Chest ({},{}) to Hopper!", x, y);
                                    println!("Chest ({},{}) deletes its item", x, y);
                                    nxtv.item_type = ItemType::None;
                                }
                            },
                            _ => {
                                if(self.direction_x != nx || self.direction_y != ny) {
                                    println!("1 Transfer to Chest ({},{}) from Hopper!", x, y);
                                    println!("Chest ({},{}) adds item ({:?}) from hopper", x, y, self.item_type);
                                    nxtv.item_type = self.item_type;
                                }
                            }
                        }
                    }
                    _ => ()
                }
            },
            BlockType::Chest => {
                match cv.block_type {
                    BlockType::Hopper => {
                        match self.item_type {
                            ItemType::None => {
                                if(self.direction_x == nx && self.direction_y == ny) {
                                    println!("Transfer to Chest ({},{}) from Hopper!", x, y);
                                    println!("Hopper ({},{}) deletes its item", x, y);
                                    nxtv.item_type = ItemType::None;
                                }
                            },
                            _ => {
                                if(self.direction_x != nx || self.direction_y != ny) {
                                    println!("Transfer from Chest ({},{}) to Hopper!", x, y);
                                    println!("Hopper ({},{}) adds item ({:?}) from chest", x, y, self.item_type);
                                    nxtv.item_type = self.item_type;
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

const N: usize = 200;

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

    Zip::indexed(&mut next_grid)
        .par_apply(|(x,y), mut nxtv| {
            let cv = grid[[x,y]];
            nxtv.block_type = cv.block_type;
            let neighbours = grid.slice(s![bgc((x as i32)-1)..=bgc((x as i32)+1), bgc((y as i32)-1)..=bgc((y as i32)+1)]);
            Zip::indexed(neighbours).apply(|(nx,ny), nv| {
                nv.update_neighbour((nx as i32) + (x as i32)-1, (ny as i32) + (y as i32)-1, x as i32, y as i32, &cv, &mut nxtv);
            })
    });

    Zip::indexed(&mut next_grid)
        .apply(|(x,y), v| {
            match v.block_type {
                BlockType::Chest => {
                    println!("CHEST WITH ({},{}) {:?}", x, y, v.item_type)
                },
                BlockType::Hopper => {
                    println!("Hopper WITH ({},{}) {:?}", x, y, v.item_type)
                },
                _ => ()
            }
    });
}
