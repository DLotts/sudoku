# Rust Sudoku text

Solves Easy and medium puzzles.  Anything that does not use guessing and backtracking -- as far as I know.

Runs in terminal mode (run -> cmd, powershell, bash)

Written in Rust for blazing speed, cross platform, and reliablility.

Edit input.csv to set the puzzle.  It is space delimited.

Todo:

- I'd like to add backtracking so it can do really hard puzzles.

- Create puzzles with one solution.  Interactively or automatically.  It's harder than I thought.

Sample output if run from a terminal:


```
        7| 4  9  1| 6     5
  2      |    6   | 3     9
         |       7|    1
 --------------------------
     5  8| 6      |       4
        3|        |    9
        6| 2      | 1  8  7
 --------------------------
  9     4|    7   |       2
  6  7   | 8  3   |
  8  1   |    4  5|

 {3} {3, 8}  7| 4  9  1| 6 {2}  5
  2 {8, 4} {1, 5}|{5}  6 {8}| 3 {4, 7}  9
 {3, 5, 4} {8, 3, 9, 4, 6} {5, 9}|{3, 5} {2, 8, 5}  7|{4, 8, 2}  1 {8}
 --------------------------
 {1, 7}  5  8| 6 {1} {3, 9}|{2} {2, 3}  4
 {1, 7, 4} {2, 4}  3|{5, 7, 1} {1, 8, 5} {8, 4}|{5, 2}  9 {6}
 {4} {4, 9}  6| 2 {5} {4, 9, 3}| 1  8  7
 --------------------------
  9 {3}  4|{1}  7 {6}|{5, 8} {3, 5, 6}  2
  6  7 {2, 5}| 8  3 {9, 2}|{5, 9, 4} {4, 5} {1}
  8  1 {2}|{9}  4  5|{9, 7} {7, 6, 3} {6, 3}
On 1 iteration, still have 46 empty cells; Found 36 new digits using only-one-missing inferencing.
...
On 3 iteration, still have 0 empty cells; Found 0 new digits using only-one-missing inferencing.
Used 3 iterations.

  3  8  7| 4  9  1| 6  2  5
  2  4  1| 5  6  8| 3  7  9
  5  6  9| 3  2  7| 4  1  8
 --------------------------
  7  5  8| 6  1  9| 2  3  4
  1  2  3| 7  8  4| 5  9  6
  4  9  6| 2  5  3| 1  8  7
 --------------------------
  9  3  4| 1  7  6| 8  5  2
  6  7  5| 8  3  2| 9  4  1
  8  1  2| 9  4  5| 7  6  3
```

Screenshot of the Tauri interaction:

![Image demonstrating the app](screenShot.png?raw=true "Screen shot")

Send me your updates/pull request! 

