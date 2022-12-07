use std::fmt::Binary;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub(crate) struct BitBoard(u64);

impl BitBoard {
    pub fn get(&self, row: usize, col: usize) -> bool {
        self.column(col) & 1 << (7 - row) != 0
    }

    #[inline]
    fn column(&self, col: usize) -> u8 {
        let bytes: [u8; 8] = unsafe { std::intrinsics::transmute(self.0) };
        bytes[col]
    }

    pub fn height(&self, col: usize) -> usize {
        8 - self.column(col).leading_zeros() as usize
    }

    pub fn set(&mut self, row: usize, col: usize, val: bool) {
        let mut bytes: [u8; 8] = unsafe { std::intrinsics::transmute(self.0) };
        if val {
            bytes[col] |= 1 << row;
        } else {
            bytes[col] &= !(1 << row);
        }
        unsafe { self.0 = std::intrinsics::transmute(bytes) };
    }

    pub fn has_four(&self) -> bool {
        let bb = self.0;
        let v = (bb << 0) & (bb << 8) & (bb << 16) & (bb << 24);
        let h = (bb << 0) & (bb << 1) & (bb << 2) & (bb << 3);
        let d = (bb << 0) & (bb << 7) & (bb << 14) & (bb << 21);
        let a = (bb << 0) & (bb << 9) & (bb << 18) & (bb << 27);

        v != 0 || h != 0 || d != 0 || a != 0
    }
}

impl Binary for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes: [u8; 8] = unsafe { std::intrinsics::transmute(self.0) };
        for byte in bytes {
            write!(f, "{:08b}", byte)?;
            write!(f, " ")?;
        }
        Ok(())
    }
}
