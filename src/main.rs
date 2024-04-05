// 9x9 sudoko puzzle.  Solve one missing row.  No error detection.
//
// TODO: 
// - naw: use ndarray crate.  
// - done Added a row+block based missing<bool>[][].  
// - answer is intersection of block, row and column missing.  Really that easy?
pub type Grid = Vec<Vec<u8>>;
pub type Hood = [bool;9];
pub type HoodDb = [Hood;9];

pub trait HoodDbOps {
    fn new() -> HoodDb;
}
impl HoodDbOps for HoodDb {
    fn new() -> HoodDb {
        [Hood::new();9]
    }
}
// This is an extension trait which allows extending other peoples structs and stuff.
pub trait HoodOps {
     fn new() -> Hood;
     fn found(&mut self, digit:u8);
     fn is_missing(&self, digit:u8)->bool;
 }
impl HoodOps for Hood {
    fn new() -> Hood{ [true;9] }
    fn found(&mut self, digit:u8) {
        self[(digit-1) as usize] = false;
    }
    fn is_missing(&self, digit:u8) -> bool {
        self[digit as usize -1]
    }
}

fn main() {
    let raw_grid=vec![
        "1?3456789",
        "4?6789123",
        "7?9123456",
        "2?1564897",
        "5?4897231",
        "8?7231564",
        "3?8672915",
        "6?2915348",
        "9?5348672",
        
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
    let  grid:Grid = raw_grid.iter()
        .map(|s|  s.bytes()
            .map(|b| if b==b'?' {0}else{b-b'0'}).collect()).collect();  
    print_grid(&grid);

    // Track the missing numbers for each hood in the game.  Hoods are rows, columns and blocks.
    // Rows are 0..8 top to bottom. Columns are 0..8 left to right. Blocks are:
    // 0|1|2
    // 3|4|5
    // 6|7|8
    // so for example the cell at the bottom right of the puzzle is row=8,col=8,blk=8
    // Center cell is row=4,col=4,blk=4

    let mut row_mis = HoodDb::new();  // 9 rows in the game
    let mut col_mis = HoodDb::new();  // 9 columns in the game
    let mut blk_mis = HoodDb::new();  // 9 blocks in the game
    const UNKNOWN:u8 = 0;

    for (i_row,row) in grid.iter().enumerate() {
        for (i_col, &digit) in row.iter().enumerate() {
            if digit!=UNKNOWN {
                col_mis[i_col].found(digit);
                row_mis[i_row].found(digit);
                blk_mis[block_from_rc(i_row,i_col)].found(digit);
            }
        }
    }
    println!("col{} row{} block{}" , 
        col_mis.iter().map(|i| (i.iter().take_while(|&&b| !b).count()+1)%10).map(|d| d.to_string()).collect::<String>(),
        row_mis.iter().map(|i| (i.iter().take_while(|&&b| !b).count()+1)%10).map(|d| d.to_string()).collect::<String>(),
        blk_mis.iter().map(|i| (i.iter().take_while(|&&b| !b).count()+1)%10).map(|d| d.to_string()).collect::<String>(),
        
    );
    //println!("col_mis={:?}" , col_mis)
}

// block index from row column index
fn block_from_rc(i_row:usize,i_col:usize) -> usize {
    i_col / 3 + (i_row/3)*3
}

// Print the puzzle
fn print_grid(grid : &Grid) {
    //grid.into_iter().for_each(|v| {v.into_iter().for_each(|d|print!("{d}"));println!("");});
    for (irow,row) in grid.into_iter().enumerate() {
        for (i,&digit) in row.into_iter().enumerate() {
            let border = if i>0&&(i)%3==0 { '|' } else {' '};
            if digit!=0 {
                print!("{border}{digit}");
            } else {
                print!("{border} ");
            }
        }
        println!("");
        if irow == 2 || irow==5 {
            println!(" -----------------");
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::HoodOps;
    use crate::HoodDb; 
    use crate::HoodDbOps;
    #[test]
    fn is_missing() {
        
        let mut row_mis = HoodDb::new();
        row_mis[2].found(1);
        assert!( row_mis[2].is_missing(2));
        assert!(! row_mis[2].is_missing(1));
    }
}

// input:
// 231
// ??? /*312*/
// 123

// Missing 2x2
// 1f.f  
// 2ff.
// 3.ff
//  Read the true dots from left to right -> 312