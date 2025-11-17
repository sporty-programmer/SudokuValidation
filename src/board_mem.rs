pub struct BoardMem {
    // 81 bits needed (one for each field of the board), so use 86 bit bitmask (the best possible case)
    low: u64,
    mid: u16,
    high: u8,
}

impl BoardMem {
    // simplifies access to the bitmask (like one dimensional array)

    pub fn new() -> BoardMem {
        BoardMem {
            low: 0,
            mid: 0,
            high: 0,
        }
    }

    pub fn get_at(&self, index: usize) -> bool {

        if index > 80 {
            panic!("Index out of bounds: {}", index);
        }

        if index < 64 {
            ((self.low >> index) & 1) != 0
        }
        else if index < 80 {
            ((self.mid >> (index - 64)) & 1) != 0
        }
        else {
            ((self.high >> (index - 80)) & 1) != 0
        }
    }

    pub fn set_at(&mut self, index: usize) {

        if index > 80 {
            panic!("Index out of bounds: {}", index);
        }

        if index < 64 {
            self.low |= 1 << index;
        }
        else if index < 80 {
            self.mid |= 1 << (index - 64);
        }
        else {
            self.high |= 1 << (index - 80);
        }
    }
}