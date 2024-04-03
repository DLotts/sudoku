// 9x9 sudoko puzzle.  Solve one missing row.  No error detection.
//
// TODO: 
// - use ndarray crate.  
// - Added a row based missing<bool>[][].  
// - answer is intersection of row and column missing.  Really that easy?
fn main() {
    let raw_grid=vec![
        "123456789",
        "912345678",
        "891234567",
        "789123456",
        "678912345",
        "567891234",
        "?????????",/*"456789123",*/
        "345678912",
        "234567891",];
    let mut grid:Vec<Vec<u8>> = raw_grid.iter()
        .map(|s|  s.bytes()
            .map(|b| if b==b'?' {0}else{b-b'0'}).collect()).collect();  
    print_grid(&grid);
    let mut missing = vec![vec![true;9];9];
    for i in 0..9 as usize {
        let row = &grid[i];
        let mut col=0;  
        for &j in row {
            if j>0 {
                missing[col][(j-1) as usize]=false;
                col+=1;
            }
        }
    }
    println!("{}" , 
        missing.iter().map(|col| col.iter().take_while(|&&b| !b).count()+1).map(|d| d.to_string()).collect::<String>()
    );
}

fn print_grid(grid : &Vec<Vec<u8>>) {
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
        if irow > 0 && (irow+1)%3==0 {
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