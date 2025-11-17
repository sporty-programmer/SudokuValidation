use crate::board_mem::BoardMem;

pub fn validate(board: [[i8; 9]; 9], unset_fields_allowed: bool) -> bool {

    let mut rows = BoardMem::new();
    let mut cols = BoardMem::new();
    let mut boxes = BoardMem::new();

    for i in 0..9 {
        for j in 0..9 {

            // field not set
            if board[i][j] == -1 {
                if !unset_fields_allowed {
                    return false;
                }
                continue;
            }

            let num = (board[i][j] - 1) as usize;

            let row_index = i * 9 + num;
            let col_index = j * 9 + num;
            let box_index = ((i / 3) * 3 + (j / 3)) * 9 + num;

            if rows.get_at(row_index) || cols.get_at(col_index) || boxes.get_at(box_index) {
                return false;
            }

            rows.set_at(row_index);
            cols.set_at(col_index);
            boxes.set_at(box_index);
        }
    }

    true
}