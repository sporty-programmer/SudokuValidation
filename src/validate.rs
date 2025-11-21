use crate::board::Board;
use crate::board_mem::BoardMem;

pub fn validate(board: Board, unset_fields_allowed: bool) -> bool {

    // memorize numbers and the area where they were found
    let mut rows = BoardMem::new(); // which numbers found in which row
    let mut cols = BoardMem::new(); // which numbers found in which col
    let mut boxes = BoardMem::new(); // which numbers found in which box

    for y in 0..9 { // row
        for x in 0..9 { // col

            // field not set
            if board.get_at(x, y) == 0 {
                if !unset_fields_allowed {
                    return false;
                }
                continue;
            }

            let index_relative = (board.get_at(x, y) - 1) as usize; // which number (-1 because it is an index)

            let row_index = y * 9 + index_relative; // which row
            let col_index = x * 9 + index_relative; // which col
            let box_index = ((y / 3) * 3 + (x / 3)) * 9 + index_relative; // which box

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