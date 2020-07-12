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
    item_type: ItemType
}

const N: usize = 2000;

// bounded grid coordinate
fn bgc(coord: i32) -> i32 {
    cmp::min((N as i32)-1, cmp::max(0, coord))
}

fn main() {
    let empty_block = Block { block_type: BlockType::None, item_type: ItemType::None };

    let mut grid = Array2::<Block>::from_elem((N, N), empty_block);

    grid[[3,3]] = Block { block_type: BlockType::Chest, item_type: ItemType::Apple };
    grid[[3,4]] = Block { block_type: BlockType::Hopper, item_type: ItemType::None };
    grid[[3,5]] = Block { block_type: BlockType::Chest, item_type: ItemType::None };

    let mut next_grid = Array2::<Block>::from_elem((N, N), empty_block);

    Zip::indexed(&mut next_grid)
        .par_apply(|(x,y), nxtv| {
            let cv = grid[[x,y]];
            let neighbours = grid.slice(s![bgc((x as i32)-1)..=bgc((x as i32)+1), bgc((y as i32)-1)..=bgc((y as i32)+1)]);
            Zip::indexed(neighbours).apply(|(x,y), nv| {
                nxtv.block_type = cv.block_type;
                match cv.block_type {
                    BlockType::Chest => {
                        match nv.block_type {
                            BlockType::Hopper => {
                                match nv.item_type {
                                    ItemType::None => {
                                        println!("Transfer from Chest ({},{}) to Hopper!", x, y);
                                        println!("Chest ({},{}) deletes its item", x, y);
                                        nxtv.item_type = ItemType::None;
                                    },
                                    _ => {
                                        println!("Transfer to Chest ({},{}) from Hopper!", x, y);
                                        println!("Chest ({},{}) adds item ({:?}) from hopper", x, y, nv.item_type);
                                        nxtv.item_type = nv.item_type;
                                    }
                                }
                            },
                            _ => ()
                        }
                    },
                    BlockType::Hopper => {
                        match nv.block_type {
                            BlockType::Chest => {
                                match nv.item_type {
                                    ItemType::None => {
                                        match nv.item_type {
                                            ItemType::None => {
                                            },
                                            _ => {
                                                println!("Transfer to Chest ({},{}) from Hopper!", x, y);
                                                println!("Hopper ({},{}) deletes its item", x, y);
                                                nxtv.item_type = ItemType::None;
                                            }
                                        }
                                    },
                                    _ => {
                                        println!("Transfer from Chest ({},{}) to Hopper!", x, y);
                                        println!("Hopper ({},{}) adds item ({:?}) from chest", x, y, nv.item_type);
                                        nxtv.item_type = nv.item_type;
                                    }
                                }
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
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
