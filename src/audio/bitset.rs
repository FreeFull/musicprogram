#[derive(Clone, Copy)]
pub struct BitSet([u8; 32]);

impl BitSet {
    pub fn new() -> BitSet {
        BitSet([0; 32])
    }
    pub fn set(&mut self, index: u8) {
        self.0[index as usize >> 3] |= 1 << (index & 0x7);
    }
    pub fn clear(&mut self, index: u8) {
        self.0[index as usize >> 3] &= !(1 << (index & 0x7));
    }
    pub fn clear_all(&mut self) {
        self.0 = [0; 32];
    }
    pub fn contains(&self, index: u8) -> bool {
        (self.0[index as usize >> 3] & (1 << (index & 0x7))) != 0
    }
}

pub struct BitSetIterator {
    set: BitSet,
    index: u16,
}

impl Iterator for BitSetIterator {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        while !self.set.contains(self.index as u8) && self.index <= 255 {
            self.index += 1;
        }
        if self.index > 255 {
            None
        } else {
            self.index += 1;
            Some((self.index - 1) as u8)
        }
    }
}

impl IntoIterator for BitSet {
    type Item = u8;

    type IntoIter = BitSetIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitSetIterator {
            set: self,
            index: 0,
        }
    }
}
