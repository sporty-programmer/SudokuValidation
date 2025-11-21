use crate::board_mem::BoardMem;

pub fn validate(board: [[i8; 9]; 9], unset_fields_allowed: bool) -> bool {

    // memorize numbers and the area where they were found
    let mut rows = BoardMem::new(); // which numbers found in which row
    let mut cols = BoardMem::new(); // which numbers found in which col
    let mut boxes = BoardMem::new(); // which numbers found in which box

    for i in 0..9 { // row
        for j in 0..9 { // col

            // field not set
            if board[i][j] == -1 {
                if !unset_fields_allowed {
                    return false;
                }
                continue;
            }

            let index_relative = (board[i][j] - 1) as usize; // which number (-1 because it is an index)

            let row_index = i * 9 + index_relative; // which row
            let col_index = j * 9 + index_relative; // which col
            let box_index = ((i / 3) * 3 + (j / 3)) * 9 + index_relative; // which box

            // invalid sudoku if already exists in row, col or box
            if rows.get_at(row_index) || cols.get_at(col_index) || boxes.get_at(box_index) {
                return false;
            }

            // safe that number has been seen in which row, col and box
            rows.set_at(row_index);
            cols.set_at(col_index);
            boxes.set_at(box_index);
        }
    }

    true
}