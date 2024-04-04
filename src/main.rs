// 9x9 sudoko puzzle.  Solve one missing row.  No error detection.
//
// TODO: 
// - use ndarray crate.  
// - Added a row based missing<bool>[][].  
// - answer is intersection of row and column missing.  Really that easy?
type Grid = Vec<Vec<u8>>;
type HoodMissing = [bool;9];

// This is an extension trait which allows extending other peoples structs and stuff.
pub trait MissingConstructor {
     fn new() -> HoodMissing;
     fn found(&mut self, digit:u8);
 }
impl MissingConstructor for HoodMissing {
    fn new() -> HoodMissing{ [true;9] }
    fn found(&mut self, digit:u8) {
        self[(digit-1) as usize] = false;
    }
}

fn main() {
    let raw_grid=vec![
        "123456789",
        "456789123",
        "789123456",
        "231564897",
        "564897231",
        "897231564",
        "348672915",
        "?????????",/*"672915348",*/
        "915348672",
        //old: "123456789","912345678","891234567","789123456","678912345","567891234","?????????",/*"456789123",*/"345678912","234567891",
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

    let mut row_mis:Vec<HoodMissing> = vec![HoodMissing::new();9];  // 9 rows in the game
    let mut col_mis:Vec<HoodMissing> = vec![HoodMissing::new();9];  // 9 columns in the game
    let mut blk_mis:Vec<HoodMissing> = vec![HoodMissing::new();9];  // 9 blocks in the game
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
    println!("{}" , 
        col_mis.iter().map(|col| col.iter().take_while(|&&b| !b).count()+1).map(|d| d.to_string()).collect::<String>()
        
    );
    println!("col_mis={:?}" , col_mis)
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
// input:
// 231
// ??? /*312*/
// 123

// Missing 2x2
// 1f.f  
// 2ff.
// 3.ff
//  Read the true dots from left to right -> 312