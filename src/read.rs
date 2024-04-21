use std::error::Error;
use crate::UNKNOWN;
use crate::Grid;

static FILE_PATH: &str = "input.csv";
pub fn read() -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let mut grid:Grid = Grid::new();
    println!("Reading file: {}\r", FILE_PATH);
    println!("in folder: {}", std::env::current_dir().unwrap().display());
    let yoread = csv::ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'#'))
        //.delimiter(b' ')
        .trim(csv::Trim::All)
        .from_path(FILE_PATH);
    for (row, result) in yoread?.records().enumerate() {
        let record = result?;
        let digits_nine = 
        if record.len()==9 {
            record.iter().collect::<Vec<_>>() // if comma separated, otherwise space(s) separated.
        } else {
            record.get(0).expect("Expecting digit data: 1 2 3 ...9").split_whitespace().collect::<Vec<_>>()
        };
        let digit_9 = digits_nine.iter()
            .map(|&s| s.as_bytes()[0])
            .map(|b| if b==b'?'|| b<b'1' || b>b'9' {UNKNOWN} else {b - b'0'}).collect(); 
        //for col in 0..9 {
        println!("{}={:?}", row, digits_nine);
        //}
        grid.push(digit_9);
    }
    Ok(grid)
}
