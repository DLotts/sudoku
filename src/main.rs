use  std::collections::HashSet;

// 9x9 sudoko puzzle.  Solve one missing row.  No error detection.
//
// TODO: 
// - naw: use ndarray crate.  
// - done Added a row+block based missing<bool>[][].  
// - answer is intersection of block, row and column missing.  Really that easy?
// - Nope, that is just one way of inference.  Must also do elimnation, leaving the only remaining possible digit.
// - lastly, some puzzles require guessing and back-tracking.
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
     fn into_set(&self) -> HashSet<u8>;
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
        self.iter().enumerate().filter(|(_,&missing)| missing).count()
    }

    // set intersection is boolean AND on neiborhoods.  
    // If one is missing from each neighbor of a cell, then it could go there.
    fn inters(self, rhs: Self) -> Self {
        self.iter().zip(rhs).map(|(&a,b)| a &&b).collect::<Vec<bool>>().try_into().unwrap_or([false;9])
        //[true, false, false, false, false, false, false, false, false]
    }
    // TODO try disjunct set using xor3 = (a ^ b ^ c) && !(a && b && c)
        // so if intersection yields one possibility, or xor3 gives one possibility!
        // hashset has method symmetric_difference
    fn inters3(self, s2: Self, s3: Self) -> Self {
        self.iter().zip(s2).zip(s3).map(|((&a,b),c)| a && b && c).collect::<Vec<bool>>().try_into().unwrap_or([false;9])
        //[true, false, false, false, false, false, false, false, false]
    }
    fn into_set(&self) -> HashSet<u8> {
        let mut out = HashSet::<u8>::new();
        self.iter().enumerate().for_each(|(i,&b)| if b { out.insert(i as u8 + 1); });
        out
    }
    //fn onlyOne() -> boolean
}

fn main() {
    let raw_grid=vec![
        // "--74916-5",  // from https://www.kennyyipcoding.com/Sudoku/sudoku.js
        // "2---6-3-9",
        // "-----7-1-",
        // "-586----4",
        // "--3----9-",
        // "--62--187",
        // "9-4-7---2",
        // "67-83----",
        // "81--45---"

        //"8--------", // from https://abcnews.go.com/blogs/headlines/2012/06/can-you-solve-the-hardest-ever-sudoku
        //"--36-----",
        //"-7--9-2--",
        //"-5---7---",
        //"----457--",
        //"---1---3-",
        //"--1----68",
        //"--85---1-",
        //"-9----4--",

        "---6----1",  //https://sudoku.com/medium/
        "9-1--4---",
        "-45---39-",
        "567-189-4",
        "-3--26---",
        "2-9-47863",
        "-5---9---",
        "7--4--61-",
        "---7-32-9",
        
    // var solution = [
    //     "387491625",
    //     "241568379",
    //     "569327418",
    //     "758619234",
    //     "123784596",
    //     "496253187",
    //     "934176852",
    //     "675832941",
    //     "812945763"
    // ]
    

        // "1?3456789",
        // "4?6789123",
        // "7?9123456",
        // "2?1564897",
        // "5?489??31",//"5?4897231",
        // "8?7231564",
        // "?????????",//"3?8672915",
        // "6?2915348",
        // "9?5348672",
        
        // "123456789",
        // "456789123",
        // "789123456",
        // "231564897",
        // "564897231",
        // "897231564",
        // "348672915",
        // "?????????",/*"672915348",*/
        // "915348672",
        // //old: "123456789","912345678","891234567","789123456","678912345","567891234","?????????",/*"456789123",*/"345678912","234567891",
    ];
    
    // copy the puzzle into an 2d array, using 0 as the unknown digit.
    const UNKNOWN:u8 = 0;
    let mut grid:Grid = raw_grid.iter()
        .map(|s|  s.bytes()
            .map(|b| if b==b'?'|| b<b'1' || b>b'9' {UNKNOWN} else {b - b'0'}).collect()).collect();  

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

    // println!("col={} row={} block={}" , 
    //     col_mis.iter().map(|i| (i.iter().enumerate().filter(|(_,&b)| b)).map(|(i,_)| (i+1).to_string()).collect::<Vec<String>>().join(",")).map(|s| format!("({s})")).collect::<String>(),
    //     row_mis.iter().map(|i| (i.iter().enumerate().filter(|(_,&b)| b)).map(|(i,_)| (i+1).to_string()).collect::<Vec<String>>().join(",")).map(|s| format!("({s})")).collect::<String>(),
    //     blk_mis.iter().map(|i| (i.iter().enumerate().filter(|(_,&b)| b)).map(|(i,_)| (i+1).to_string()).collect::<Vec<String>>().join(",")).map(|s| format!("({s})")).collect::<String>(),
    // );
    //println!("col_mis={:?}" , col_mis)
 
    // solve the puzzle!
    // keep missing sets for each grid cell.
    let mut grid_mis = [[MisSet::new();9];9];
    // next grid allows us to change the data while iterating the original.
    let mut next_grid;
    let mut still_changing ;
    let mut unknown_cell_count = 1;
    let mut loop_count = 0;
    while unknown_cell_count > 0  {
        print_notes_grid(&grid, true, row_mis, col_mis, blk_mis);
        still_changing = false;
        unknown_cell_count = 0;
        next_grid = grid.clone();
        for (i_row,row) in grid.iter().enumerate() {
            for (i_col,&digit) in row.into_iter().enumerate() {
                if digit==0 {
                    unknown_cell_count += 1;
                    //let missing = row_mis[i_row].inters(col_mis[i_col]).inters(blk_mis[block_from_rc(i_row,i_col)]).into_set();
                    let missing_mis_set = row_mis[i_row].inters3(col_mis[i_col],blk_mis[block_from_rc(i_row,i_col)]);
                    let missing = missing_mis_set.into_set();
                    if missing.len() == 0  {
                        println!("Discovered a cell with no possible solutions after {} iterations.", loop_count);
                    }
                    if missing.len()==1 {
                        // Only one possible solution, put into the grid and update the missing hoods arrays.
                        let only_digit = *missing.iter().next().unwrap();
                        next_grid[i_row][i_col] = only_digit;
                        col_mis[i_col].found(only_digit);
                        row_mis[i_row].found(only_digit);
                        blk_mis[block_from_rc(i_row,i_col)].found(only_digit);
                        grid_mis[i_row][i_col]=MisSet::new_all_found();
                        still_changing = true;
                    } else {
                        grid_mis[i_row][i_col]=missing_mis_set;
                    }
                }
            }
        }
        loop_count+=1;
        grid = next_grid;
        if loop_count > 100 {
            println!("After {} iterations, no solutions.  Perhaps it has no or multiple solutions?", loop_count);
            break;
        }
        // Second inference type, elimination per neighboring
        if unknown_cell_count > 0 && ! still_changing {
            unknown_cell_count = 0;
            for (i_row,row) in grid_mis.iter().enumerate() {
                for (i_col,existing_mis_set) in row.into_iter().enumerate() {
                    if grid[i_row][i_col]==0 {
                        unknown_cell_count += 1;
                        let mut mis_set = existing_mis_set.clone();
                        // take all the same missing digits along the ROW
                        for (take_col, &take_mis_set) in  row.iter().enumerate() {
                            if take_col != i_col { // skip yourself
                                mis_set.remove(take_mis_set);
                            }
                        }
                        if mis_set.len() !=1 { // do col then blk
                            // take all the same missing digits along the COLUMN
                            mis_set = existing_mis_set.clone();
                            for take_row  in  0..=8 {
                                if take_row != i_row { // skip yourself
                                    mis_set.remove(grid_mis[take_row][i_col]);
                                }
                            }

                        }
                        if mis_set.len() !=1 {
                            // take all the same missing digits in the BLOCK
                            let i_blk = block_from_rc(i_row, i_col);
                            let rc_from_block = rc_from_block(i_blk);
                            mis_set = existing_mis_set.clone();
                            for (take_row,take_col)  in  rc_from_block {
                                if !(take_row as usize == i_row && take_col as usize == i_col) { // skip yourself
                                    mis_set.remove(grid_mis[take_row as usize][take_col as usize]);
                                }
                            }

                        }
                        
                        if mis_set.len()==1 {
                            // Only one possible solution, put into the grid and update the missing hoods arrays.
                            let only_digit = (mis_set.iter().position(|&b| b).unwrap() + 1) as u8;
                            grid[i_row][i_col] = only_digit;
                            col_mis[i_col].found(only_digit);
                            row_mis[i_row].found(only_digit);
                            blk_mis[block_from_rc(i_row,i_col)].found(only_digit);
                            still_changing = true;
                        }
                    }
                }
            }
         }
         if  unknown_cell_count>0 && ! still_changing {
            println!("After {} iterations, still have {} empty cells and elimination inference found nothing. Thats all I can do", loop_count, unknown_cell_count);
            break;
         }
    }

    // print the solution!
    println!("Used {} iterations.", loop_count);
    print_notes_grid(&grid, false, row_mis, col_mis, blk_mis);
    
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
                    let missing = row_mis[i_row].inters3(col_mis[i_col], blk_mis[block_from_rc(i_row,i_col)]).into_set();
                    print!("{:?}",missing);
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
    use std::collections::HashSet;

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
        assert_eq!(a.into_set(),  HashSet::from([2,3,8,9]) , "presense in the set means not found");
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