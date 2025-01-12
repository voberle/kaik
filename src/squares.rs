#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub fn get_rank(self) -> usize {
        self as usize / 8 + 1
    }

    pub fn get_file(self) -> char {
        (self as u8 % 8 + b'a') as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rank() {
        assert_eq!(Square::A1.get_rank(), 1);
        assert_eq!(Square::C3.get_rank(), 3);
        assert_eq!(Square::H8.get_rank(), 8);
    }

    #[test]
    fn test_get_file() {
        assert_eq!(Square::A1.get_file(), 'a');
        assert_eq!(Square::B5.get_file(), 'b');
        assert_eq!(Square::C1.get_file(), 'c');
        assert_eq!(Square::D8.get_file(), 'd');
        assert_eq!(Square::E7.get_file(), 'e');
        assert_eq!(Square::F3.get_file(), 'f');
        assert_eq!(Square::G6.get_file(), 'g');
        assert_eq!(Square::H8.get_file(), 'h');
    }
}
