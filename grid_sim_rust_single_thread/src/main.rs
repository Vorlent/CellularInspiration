use ndarray::{s, Array2, Zip};
use std::cmp;

#[derive(Copy, Clone, Debug, PartialEq)]
enum BlockType {
    None,
    Chest,
    Hopper
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum ItemType {
    None,
    Apple
}

#[derive(Copy, Clone)]
struct Block {
    block_type: BlockType,
    item_type: ItemType,
    direction_x: i32,
    direction_y: i32
}

impl Block {
    fn update(&self, x: i32, y: i32, grid: &Array2::<Block>, next_grid: &mut Array2::<Block>) -> () {

        match self.block_type {
            // only hoppers have logic in the single threaded version
            BlockType::Hopper => {
                let neighbours = grid.slice(s![bgc((x as i32)-1)..=bgc((x as i32)+1), bgc((y as i32)-1)..=bgc((y as i32)+1)]);
                Zip::indexed(neighbours).apply(|(nx,ny), _nnv| {
                    let cnx = (nx as i32) + (x as i32)-1;
                    let cny = (ny as i32) + (y as i32)-1;

                    let neighbour = grid[[cnx as usize, cny as usize]];
                    match neighbour.block_type {
                        BlockType::Chest => {
                            // check if chest is empty
                            if self.direction_x == cnx && self.direction_y == cny && neighbour.item_type == ItemType::None {
                                // check if hopper is not empty
                                if self.item_type != ItemType::None {
                                    println!("insert item into empty chest");
                                    {
                                        let mut nv = &mut next_grid[[cnx as usize, cny as usize]];
                                        nv.item_type = self.item_type;
                                    }
                                    {
                                        let mut nxtv = &mut next_grid[[x as usize, y as usize]];
                                        nxtv.item_type = ItemType::None;
                                    }
                                }
                            }
                            // check if chest is not empty
                            if (self.direction_x != cnx || self.direction_y != cny) && neighbour.item_type != ItemType::None {
                                // check if hopper is empty
                                if self.item_type == ItemType::None {
                                    println!("pull item from full chest");
                                    {
                                        let mut nv = &mut next_grid[[cnx as usize, cny as usize]];
                                        nv.item_type = ItemType::None;
                                    }
                                    {
                                        let mut nxtv = &mut next_grid[[x as usize, y as usize]];
                                        nxtv.item_type = neighbour.item_type;
                                    }
                                }
                            }
                        },
                        _ => ()
                    }
                })
            },
            _ => ()
        }
    }
}

const N: usize = 6;

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
    Zip::indexed(grid)
        .apply(|(x,y), cv| {
            {
                let mut nxtv = &mut next_grid[[x as usize, y as usize]];
                // copy all current data to the new block
                nxtv.block_type = cv.block_type;
                nxtv.item_type = cv.item_type;
                nxtv.direction_x = cv.direction_x;
                nxtv.direction_y = cv.direction_y;
            }
    });
    Zip::indexed(grid)
        .apply(|(x,y), cv| {
            cv.update(x as i32, y as i32, grid, next_grid);
    });
}
