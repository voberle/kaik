#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const EMPTY: BitBoard = BitBoard(0);

    pub fn is_set(self, index: u8) -> bool {
        self.0 & (1 << index) != 0
    }

    pub fn set(&mut self, index: u8) {
        self.0 |= 1 << index;
    }

    pub fn print(self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let index = rank * 8 + file;
                if file == 0 {
                    print!("  {rank} ");
                }
                print!(" {}", u8::from(self.is_set(index)));
            }
            println!();
        }
        println!("     a b c d e f g h");
    }
}

impl From<u64> for BitBoard {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::squares::Square;

    use super::*;

    #[test]
    fn test_create_set() {
        use Square::*;
        let white_pawns: BitBoard = 71776119061217280.into();

        let mut b = BitBoard::EMPTY;
        for square in [A2, B2, C2, D2, E2, F2, G2, H2] {
            b.set(square.into());
        }
        assert_eq!(b, white_pawns);
    }
}
