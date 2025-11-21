pub struct BoardMem {
    // 81 bits needed (one for each field of the board), so use 86 bit bitmask (the best possible size)
    v1: u64,
    v2: u16,
    v3: u8,
}

impl BoardMem {
    // simplifies access to the bitmask (like one dimensional array)

    pub fn new() -> BoardMem {
        BoardMem {
            v1: 0,
            v2: 0,
            v3: 0,
        }
    }

    pub fn get_at(&self, index: usize) -> bool {

        if index > 80 {
            panic!("Index out of bounds: {}", index);
        }

        if index < 64 {
            (self.v1 & (1u64 << index)) != 0
        }
        else if index < 80 {
            (self.v2 & (1u16 << (index - 64))) != 0
        }
        else {
            (self.v3 & (1u8 << (index - 80))) != 0
        }
    }

    pub fn set_at(&mut self, index: usize) {

        if index > 80 {
            panic!("Index out of bounds: {}", index);
        }

        if index < 64 {
            self.v1 |= 1u64 << index;
        }
        else if index < 80 {
            self.v2 |= 1u16 << (index - 64);
        }
        else {
            self.v3 |= 1u8 << (index - 80);
        }
    }
}