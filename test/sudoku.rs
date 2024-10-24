struct Board {
    grid: [[u8; 9]; 9],
}

impl Board {
    fn new(grid: [[u8; 9]; 9]) -> Board {
        Board { grid }
    }

    fn is_valid(&self, row: usize, col: usize, num: u8) -> bool {
        // Check if num is valid at position (row, col)

        // Check row
        for c in 0..9 {
            if self.grid[row][c] == num {
                return false;
            }
        }

        // Check column
        for r in 0..9 {
            if self.grid[r][col] == num {
                return false;
            }
        }

        // Check 3x3 subgrid
        let start_row = (row / 3) * 3;
        let start_col = (col / 3) * 3;

        for r in start_row..start_row + 3 {
            for c in start_col..start_col + 3 {
                if self.grid[r][c] == num {
                    return false;
                }
            }
        }

        true
    }

    fn find_empty(&self) -> Option<(usize, usize)> {
        // Find an empty cell (represented by 0)
        for r in 0..9 {
            for c in 0..9 {
                if self.grid[r][c] == 0 {
                    return Some((r, c));
                }
            }
        }
        None
    }

    fn solve(&mut self) -> bool {
        if let Some((row, col)) = self.find_empty() {
            for num in 1..=9 {
                if self.is_valid(row, col, num) {
                    self.grid[row][col] = num;
                    self.print_board();

                    if self.solve() {
                        return true;
                    }

                    // Backtrack
                    self.grid[row][col] = 0;
                }
            }
            false
        } else {
            true
        }
    }

    fn print_board(&self) {
        // Move cursor to 0,0
        print!("\x1b[H");

        for r in 0..9 {
            if r % 3 == 0 && r != 0 {
                println!("------+-------+------");
            }
            for c in 0..9 {
                if c % 3 == 0 && c != 0 {
                    print!("| ");
                }
                if self.grid[r][c] == 0 {
                    print!(". ");
                } else {
                    print!("{} ", self.grid[r][c]);
                }
            }
            println!();
        }
        println!();
    }
}

fn main() {
    // Initial board (0 represents empty cells)
    let initial_grid = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        //
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        //
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    let mut board = Board::new(initial_grid);

    board.print_board();

    if board.solve() {
        println!("Sudoku solved!");
    } else {
        println!("No solution exists.");
    }
}
