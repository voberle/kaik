#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitBoard(u64);

impl BitBoard {
    fn is_set(self, index: u8) -> bool {
        let mask = 1 << index;
        self.0 & mask != 0
    }

    pub fn print(self) {
        for r in (0..8).rev() {
            for f in 0..8 {
                let index = r * 8 + f;
                print!("{}", u8::from(self.is_set(index)));
            }
            println!();
        }
    }
}

impl From<u64> for BitBoard {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
