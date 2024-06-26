// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Window;
//use std::collections::HashSet;

pub const UNKNOWN:u8 = 0;

mod read;

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![greet,the_time,solve_it,load_ui])
  .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(_name: &str) -> String {
  format!("Feel free to enter a puzzle, or load one from a file.  Click solve when ready.")
}

#[tauri::command]
fn the_time() -> String {
    // Reserved for future use
    format!("")
}

#[tauri::command]
fn solve_it(grid:Grid, window: Window) -> Grid {
    //let grid = crate::read::read().expect("Bad input data.");
    solve(grid, window)
}

#[tauri::command]
fn load_ui(file:String) -> Grid {
    crate::read::read(file).expect("Bad input data.")
}

// 9x9 sudoko puzzle.  
// - inference 1: only-one-missing is intersection of block, row and column has exactly one missing digit.
// - inference 2: Elimnation-per-neighborhood, find missing digit that is unique in one cell for a row,col, or block.
// - inference 3: some difficult puzzles require guessing and back-tracking.  That is to do.
pub type Grid = Vec<Vec<u8>>;
pub type MisSet = [bool;9];
pub type HoodMisSet = [MisSet;9];

pub trait HoodMisSetOps {
    fn new() -> HoodMisSet;
}
impl HoodMisSetOps for HoodMisSet {
    fn new() -> HoodMisSet {
        [MisSet::new();9]
    }
}
// This is an extension trait which allows extending other peoples structs and stuff.
pub trait MisSetOps {
     fn new() -> MisSet;
     fn new_all_found() -> MisSet;
     fn found(&mut self, digit:u8);
     fn is_missing(&self, digit:u8)->bool;
     fn inters(self, rhs: Self) -> Self;
     fn inters3(self, s2: Self, s3: Self) -> Self ;
     fn remove(&mut self, take:MisSet);
     fn len(&self) -> usize;
     fn to_string(&self) -> String;
 }
impl MisSetOps for MisSet {
    fn new() -> MisSet{ [true;9] }
    fn new_all_found() -> MisSet{ [false;9] }

    fn found(&mut self, digit:u8) {
        self[(digit-1) as usize] = false;
    }
    fn is_missing(&self, digit:u8) -> bool {
        self[digit as usize -1]
    }
    fn remove(&mut self, take:MisSet) {
        take.iter().enumerate().filter(|(_,&missing)| missing).for_each(|(i,_)|self[i] = false);
    }
    fn len(&self) -> usize {
        self.iter().filter(|&&missing| missing).count()
    }

    // set intersection is boolean AND on neiborhoods.  
    // If one is missing from each neighbor of a cell, then it could go there.
    fn inters(self, rhs: Self) -> Self {
        self.iter().zip(rhs).map(|(&a,b)| a &&b).collect::<Vec<bool>>().try_into().unwrap_or([false;9])
        //[true, false, false, false, false, false, false, false, false]
    }
    
    // intersection on three sets
    fn inters3(self, s2: Self, s3: Self) -> Self {
        self.iter().zip(s2).zip(s3).map(|((&a,b),c)| a && b && c).collect::<Vec<bool>>().try_into().unwrap_or([false;9])
        //[true, false, false, false, false, false, false, false, false]
    }
    fn to_string(&self) -> String {
        "{".to_owned()+&self.iter().enumerate().filter(|(_,&b)| b ).map(|(i,_)|{ (i as u8 + 1).to_string() }).collect::<Vec<String>>().join(",") + "}"
    }
}

// datum to pass to the UI for display
#[derive(Clone, serde::Serialize)]
struct CellUpdate {
    row: u8,
    col: u8,
    digit: u8,
}

/////////////////////////
/// solve the Rubiks cube -- NO! the Sudoku!
fn solve(mut grid:Grid, window: Window) -> Grid {
    // grid is the puzzle in a 2d array, using 0 as the unknown digit.
    // Track the missing numbers for each hood in the game.  Hoods are rows, columns and blocks.
    // Rows are 0..8 top to bottom. Columns are 0..8 left to right. Blocks are:
    // 0|1|2
    // 3|4|5
    // 6|7|8
    // so for example the cell at the bottom right of the puzzle is row=8,col=8,blk=8
    // Center cell is row=4,col=4,blk=4

    let mut row_mis = HoodMisSet::new();  // 9 rows in the game
    let mut col_mis = HoodMisSet::new();  // 9 columns in the game
    let mut blk_mis = HoodMisSet::new();  // 9 blocks in the game

    // preset all the missing digits in the hoods
    for (i_row,row) in grid.iter().enumerate() {
        for (i_col, &digit) in row.iter().enumerate() {
            if digit!=UNKNOWN {
                col_mis[i_col].found(digit);
                row_mis[i_row].found(digit);
                blk_mis[block_from_rc(i_row,i_col)].found(digit);
            }
        }
    }

    // print the before puzzle.
    print_notes_grid(&grid, false, row_mis, col_mis, blk_mis);

    // solve the puzzle!
    // keep missing sets for each grid cell.  Needed for elimination inference, when one-possible inference is exhausted.
    let mut grid_mis = [[MisSet::new();9];9];
    // next grid allows us to change the data while iterating the original.
    let mut next_grid;
    let mut loop_change_count ;
    let mut unknown_cell_count = 1;
    let mut loop_count = 0;
    while unknown_cell_count > 0  {
        print_notes_grid(&grid, true, row_mis, col_mis, blk_mis);
        loop_change_count = 0;
        unknown_cell_count = 0;
        next_grid = grid.clone();
        for (i_row,row) in grid.iter().enumerate() {
            for (i_col,&digit) in row.into_iter().enumerate() {
                if digit==0 {
                    unknown_cell_count += 1;
                    //let missing = row_mis[i_row].inters(col_mis[i_col]).inters(blk_mis[block_from_rc(i_row,i_col)]).into_set();
                    let missing_mis_set = row_mis[i_row].inters3(col_mis[i_col],blk_mis[block_from_rc(i_row,i_col)]);
                    if missing_mis_set.len() == 0  {
                        println!("Discovered a cell with no possible solutions after {} iterations. row={i_row} col={i_col} {}", loop_count,missing_mis_set.to_string());
                        println!("row_mis {} col_mis {} blk_mis {}", row_mis[i_row].to_string(),col_mis[i_col].to_string(),blk_mis[block_from_rc(i_row,i_col)].to_string());
                    }
                    if missing_mis_set.len()==1 {
                        // Only one possible solution, put into the grid and update the missing hoods arrays.
                        let only_digit = missing_mis_set.iter().position(|&b|b).unwrap() as u8 +1;
                        next_grid[i_row][i_col] = only_digit;
                        col_mis[i_col].found(only_digit);
                        row_mis[i_row].found(only_digit);
                        blk_mis[block_from_rc(i_row,i_col)].found(only_digit);
                        grid_mis[i_row][i_col]=MisSet::new_all_found();
                        loop_change_count += 1 ;
                        window.emit("solve_digit", CellUpdate { row:i_row as u8, col:i_col as u8, digit:only_digit }).unwrap();
                    } else {
                        grid_mis[i_row][i_col]=missing_mis_set;
                    }
                }
            }
        }
        loop_count+=1;
        grid = next_grid;
        if loop_count > 100 {
            println!("After {} iterations, no solutions.  Perhaps there is a bug?", loop_count);
            break;
        }
        println!("On {} iteration, still have {} empty cells; Found {} new digits using only-one-missing inferencing.", loop_count, unknown_cell_count, loop_change_count);

        // Second inference type, elimination per neighborhood
        if unknown_cell_count > 0 && loop_change_count==0 {
            unknown_cell_count = 0;
            for i_row in 0..=8 {
                for i_col in 0..=8 {
                    if grid[i_row][i_col]==0 {
                        unknown_cell_count += 1;
                        let mut mis_set = grid_mis[i_row][i_col].clone();
                        // take all the same missing digits along the ROW
                        for take_col in 0..=8 {
                            if take_col != i_col && grid[i_row][take_col]==0  { // skip yourself  and filled digits
                                mis_set.remove(grid_mis[i_row][take_col]);
                            }
                        }
                        if mis_set.len() !=1 { // do col then blk
                            // take all the same missing digits along the COLUMN
                            mis_set = grid_mis[i_row][i_col].clone();
                            for take_row  in  0..=8 {
                                if take_row != i_row && grid[take_row][i_col]==0{ // skip yourself  and filled digits
                                    mis_set.remove(grid_mis[take_row][i_col]);
                                }
                            }

                        }
                        if mis_set.len() !=1 {
                            // take all the same missing digits in the BLOCK
                            let i_blk = block_from_rc(i_row, i_col);
                            let rc_from_block = rc_from_block(i_blk);
                            mis_set = grid_mis[i_row][i_col].clone();
                            for (take_row,take_col)  in  rc_from_block {
                                if !(take_row as usize == i_row && take_col as usize == i_col) 
                                    && grid[take_row as usize][take_col as usize]==0 { // skip yourself and filled digits
                                    mis_set.remove(grid_mis[take_row as usize][take_col as usize]);
                                }
                            }

                        }
                        
                        if mis_set.len()==1 {
                            // Only one possible solution, put into the grid and update the missing hoods arrays.
                            let only_digit = (mis_set.iter().position(|&b| b).unwrap() + 1) as u8;
                            let i_block = block_from_rc(i_row,i_col);
                            grid[i_row][i_col] = only_digit;
                            col_mis[i_col].found(only_digit);
                            row_mis[i_row].found(only_digit);
                            blk_mis[i_block].found(only_digit);
                            grid_mis[i_row][i_col].found(only_digit);
                            for i in 0..=8 {
                                grid_mis[i][i_col].found(only_digit);
                                grid_mis[i_row][i].found(only_digit);
                            }
                            for (r,c) in rc_from_block(i_block) {
                                grid_mis[r as usize][c as usize].found(only_digit);
                            }
                            loop_change_count += 1;
                            window.emit("solve_digit", CellUpdate { row:i_row as u8, col:i_col as u8, digit:only_digit }).unwrap();
                        }
                    }
                }
            }
            if loop_change_count > 0 {
                println!("After {} iterations, {} empty cells and elimination inference found {}", loop_count, unknown_cell_count, loop_change_count);
             }
         }
         if  unknown_cell_count>0 && loop_change_count==0 {
            println!("After {} iterations, still have {} empty cells and elimination inference found nothing. Thats all I can do", loop_count, unknown_cell_count);
            break;
         }
    }

    // print the solution!
    println!("Used {} iterations.", loop_count);
    print_notes_grid(&grid, false, row_mis, col_mis, blk_mis);
    
    return grid;
}

// Determine the block (3x3) index from row,column index
fn block_from_rc(i_row:usize,i_col:usize) -> usize {
    i_col / 3 + (i_row/3)*3
}
// Determine  row,column indexes from the block (3x3) index
fn rc_from_block(i_block:usize) -> [(u8,u8);9] {
    let offset = match i_block {
        0 => (0u8,0u8),
        1 => (0,3),
        2 => (0,6),
        3 => (3,0),
        4 => (3,3),
        5 => (3,6),
        6 => (6,0),
        7 => (6,3),
        8 => (6,6),
        _ => panic!("Block index too large"),
    };
    [(0,0),(0,1),(0,2),(1,0),(1,1),(1,2),(2,0),(2,1),(2,2),]
        .iter().map(|(r,c)| (r+offset.0, c+offset.1)).collect::<Vec<(u8,u8)>>().try_into().unwrap()
}

fn print_notes_grid(grid: &Vec<Vec<u8>>, show_notes:bool, row_mis: HoodMisSet, col_mis: HoodMisSet, blk_mis: HoodMisSet) {
    // print the grid with sets of missing digits
    println!("");
    for (i_row,row) in grid.iter().enumerate() {
        for (i_col,&digit) in row.into_iter().enumerate() {
            print!("{}", if i_col>0&&(i_col)%3==0 { '|' } else {' '});
            if digit!=0 {
                print!(" {digit}");
            } else {
                if show_notes {
                    let missing = row_mis[i_row].inters3(col_mis[i_col], blk_mis[block_from_rc(i_row,i_col)]).to_string();
                    print!("{}",missing);
                } else {
                    print!("  ");
                }

            }
        }
        println!("");
        if i_row == 2 || i_row==5 {
            println!(" --------------------------");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MisSetOps;
    use crate::MisSet;
    use crate::HoodMisSet; 
    use crate::HoodMisSetOps;
    #[test]
    fn is_missing() {
        let mut row_mis = HoodMisSet::new();
        row_mis[2].found(1);
        assert!( row_mis[2].is_missing(2));
        assert!(! row_mis[2].is_missing(1));
    }
    #[test]
    fn intersection() {
        let mut a = MisSet::new();
        let mut b = MisSet::new();
        let mut c = MisSet::new();
        assert_eq!(a.inters(b).inters(c), [true, true, true,true, true, true,true, true, true], "true means missing");
        c.found(1);
        assert_eq!(a.inters(b).inters(c), [false, true, true,true, true, true,true, true, true], "true means missing");
        a.found(4);
        assert_eq!(a.inters(b).inters(c), [false, true, true,false, true, true,true, true, true], "true means missing");

        a.found(1);                  a.found(4);a.found(5);a.found(6);a.found(7);
        b.found(1);b.found(3);                  b.found(5);b.found(6);b.found(7);
        c.found(1);c.found(3);c.found(4);                  c.found(6);c.found(7);

        assert_eq!(a.inters(b).inters(c), [false, true, false, false, false, false, false, true, true], "true means missing");
        c.found(2);
        assert_eq!(a.inters(b).inters(c), [false, false, false, false, false, false, false, true, true], "true means missing");
        c.found(8);
        assert_eq!(a.inters(b).inters(c), [false, false, false, false, false, false, false, false, true], "true means missing");
        c.found(9);
        assert_eq!(a.inters(b).inters(c), [false, false, false, false, false, false, false, false, false], "true means missing");

    }
    #[test]
    fn bool9_to_set() {
        let mut a = MisSet::new();
        a.found(1);a.found(4);a.found(5);a.found(6);a.found(7);
        assert_eq!(a.to_string(),  "{2,3,8,9}" , "presense in the set means not found");
    }

    #[test]
    fn len() {
        let mut a = MisSet::new();
        assert_eq!(9, a.len());
        a.found(1);                  a.found(4);a.found(5);a.found(6);a.found(7);
        assert_eq!(4, a.len());

    }       
    #[test]
    fn remove() {
        let mut a = MisSet::new(); // all missing/true
        let mut b = MisSet::new();
        a.found(1);/*2 true */                     a.found(4);a.found(5);a.found(6);a.found(7);
        b.found(1);/*remove 2 */ b.found(3);                  b.found(5);b.found(6);b.found(7);
        a.remove(b); // missing/true removes coresponding missing/true, founds/false don't change.
        assert_eq!(a, [false, false, true, false, false, false, false, false, false], "true means missing");
    }       
}