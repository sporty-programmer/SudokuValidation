pub struct Board {
    // 324 bits needed (4 for each field of the board), so use 328 bit bitmask (the best possible size)
    v1: u128,
    v2: u128,
    v3: u64,
    v4: u8,
}

impl Board {
    // simplifies access to the bitmask (like one dimensional array)

    pub fn empty() -> Board {
        Board {
            v1: 0u128,
            v2: 0u128,
            v3: 0u64,
            v4: 0u8,
        }
    }

    pub fn new(board_raw: [[char; 9]; 9]) -> Board {
        let mut board = Board::empty();
        for y in 0..9 {
            for x in 0..9 {
                if board_raw[y][x] == ' ' {
                    continue;
                }
                if !"123456789".contains(board_raw[y][x]) {
                    panic!("Invalid board");
                }
                board.set_at(x, y, board_raw[y][x].to_digit(10).unwrap() as u8);
            }
        }
        board
    }

    pub fn get_at(&self, index_x: usize, index_y: usize) -> u8 {

        let index_bit = (index_y * 9 + index_x) * 4;

        if index_bit > 323 {
            panic!("Index out of bounds: ({index_x}|{index_y})");
        }

        let bounder: u8 = 0b00001111;

        if index_bit < 128 {
            let size = size_of_val(&self.v1) * 8;
            let index_local = index_bit;
            let shift_by = size - 4 - index_local;
            ((self.v1 >> shift_by) as u8) & bounder
        }
        else if index_bit < 256 {
            let size = size_of_val(&self.v2) * 8;
            let index_local = index_bit - 128;
            let shift_by = size - 4 - index_local;
            ((self.v2 >> shift_by) as u8) & bounder
        }
        else if index_bit < 320 {
            let size = size_of_val(&self.v3) * 8;
            let index_local = index_bit - 256;
            let shift_by = size - 4 - index_local;
            ((self.v3 >> shift_by) as u8) & bounder
        }
        else {
            let size = size_of_val(&self.v4) * 8;
            let index_local = index_bit - 320;
            let shift_by = size - 4 - index_local;
            (self.v4 >> shift_by) & bounder
        }
    }

    pub fn set_at(&mut self, index_x: usize, index_y: usize, value: u8) {

        let index_bit = (index_y * 9 + index_x) * 4;

        if index_bit > 323 {
            panic!("Index out of bounds: {}", index_bit);
        }
        if value > 0b00001111 {
            panic!("Value out of bounds: {}", value);
        }

        if index_bit < 128 {
            let size = size_of_val(&self.v1) * 8;
            let index_local = index_bit;
            let shift_by = size - 4 - index_local;
            let value_shifted = (value as u128) << shift_by;
            self.v1 |= value_shifted;
        }
        else if index_bit < 256 {
            let size = size_of_val(&self.v2) * 8;
            let index_local = index_bit - 128;
            let shift_by = size - 4 - index_local;
            let value_shifted = (value as u128) << shift_by;
            self.v2 |= value_shifted;
        }
        else if index_bit < 320 {
            let size = size_of_val(&self.v3) * 8;
            let index_local = index_bit - 256;
            let shift_by = size - 4 - index_local;
            let value_shifted = (value as u64) << shift_by;
            self.v3 |= value_shifted;
        }
        else {
            let size = size_of_val(&self.v4) * 8;
            let index_local = index_bit - 320;
            let shift_by = size - 4 - index_local;
            let value_shifted = (value) << shift_by;
            self.v4 |= value_shifted;
        }
    }
}
