//! Square enumeration.
//! Converting the square to an integer gives us the position of the corresponding bit in the bitboard.

use std::fmt::Display;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl From<Square> for u8 {
    fn from(square: Square) -> Self {
        square as u8
    }
}

impl From<u8> for Square {
    fn from(val: u8) -> Self {
        assert!(val < 64);
        // The safe alternative would be to use a match, but seems a big match like this would be slower.
        unsafe { std::mem::transmute(val) }
    }
}

impl From<u32> for Square {
    #[allow(clippy::cast_possible_truncation)]
    fn from(val: u32) -> Self {
        assert!(val < 64);
        unsafe { std::mem::transmute(val as u8) }
    }
}

impl From<(char, usize)> for Square {
    fn from(value: (char, usize)) -> Self {
        // Not the most efficient, but only for debugging a priori.
        format!("{}{}", value.0, value.1)
            .as_str()
            .try_into()
            .unwrap()
    }
}

impl TryFrom<&str> for Square {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "A1" => Ok(Square::A1),
            "B1" => Ok(Square::B1),
            "C1" => Ok(Square::C1),
            "D1" => Ok(Square::D1),
            "E1" => Ok(Square::E1),
            "F1" => Ok(Square::F1),
            "G1" => Ok(Square::G1),
            "H1" => Ok(Square::H1),
            "A2" => Ok(Square::A2),
            "B2" => Ok(Square::B2),
            "C2" => Ok(Square::C2),
            "D2" => Ok(Square::D2),
            "E2" => Ok(Square::E2),
            "F2" => Ok(Square::F2),
            "G2" => Ok(Square::G2),
            "H2" => Ok(Square::H2),
            "A3" => Ok(Square::A3),
            "B3" => Ok(Square::B3),
            "C3" => Ok(Square::C3),
            "D3" => Ok(Square::D3),
            "E3" => Ok(Square::E3),
            "F3" => Ok(Square::F3),
            "G3" => Ok(Square::G3),
            "H3" => Ok(Square::H3),
            "A4" => Ok(Square::A4),
            "B4" => Ok(Square::B4),
            "C4" => Ok(Square::C4),
            "D4" => Ok(Square::D4),
            "E4" => Ok(Square::E4),
            "F4" => Ok(Square::F4),
            "G4" => Ok(Square::G4),
            "H4" => Ok(Square::H4),
            "A5" => Ok(Square::A5),
            "B5" => Ok(Square::B5),
            "C5" => Ok(Square::C5),
            "D5" => Ok(Square::D5),
            "E5" => Ok(Square::E5),
            "F5" => Ok(Square::F5),
            "G5" => Ok(Square::G5),
            "H5" => Ok(Square::H5),
            "A6" => Ok(Square::A6),
            "B6" => Ok(Square::B6),
            "C6" => Ok(Square::C6),
            "D6" => Ok(Square::D6),
            "E6" => Ok(Square::E6),
            "F6" => Ok(Square::F6),
            "G6" => Ok(Square::G6),
            "H6" => Ok(Square::H6),
            "A7" => Ok(Square::A7),
            "B7" => Ok(Square::B7),
            "C7" => Ok(Square::C7),
            "D7" => Ok(Square::D7),
            "E7" => Ok(Square::E7),
            "F7" => Ok(Square::F7),
            "G7" => Ok(Square::G7),
            "H7" => Ok(Square::H7),
            "A8" => Ok(Square::A8),
            "B8" => Ok(Square::B8),
            "C8" => Ok(Square::C8),
            "D8" => Ok(Square::D8),
            "E8" => Ok(Square::E8),
            "F8" => Ok(Square::F8),
            "G8" => Ok(Square::G8),
            "H8" => Ok(Square::H8),
            _ => Err("Invalid square"),
        }
    }
}

impl Square {
    pub fn new(rank: u8, file: u8) -> Self {
        ((rank << 3) + file).into()
    }

    pub fn get_rank(self) -> u8 {
        (self as u8 & 56) >> 3
    }

    pub fn get_file(self) -> u8 {
        self as u8 & 7
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rank = self.get_rank() + 1;
        let file = (self.get_file() + b'a') as char;
        write!(f, "{file}{rank}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        assert_eq!(Into::<Square>::into(32u8), Square::A5);
        assert_eq!(Square::new(4, 0), Square::A5);
        assert_eq!(Square::new(2, 2), Square::C3);
    }

    #[test]
    fn test_get_rank() {
        assert_eq!(Square::A1.get_rank(), 0);
        assert_eq!(Square::B3.get_rank(), 2);
        assert_eq!(Square::H8.get_rank(), 7);
    }

    #[test]
    fn test_get_file() {
        assert_eq!(Square::A1.get_file(), 0);
        assert_eq!(Square::B5.get_file(), 1);
        assert_eq!(Square::C1.get_file(), 2);
        assert_eq!(Square::D8.get_file(), 3);
        assert_eq!(Square::E7.get_file(), 4);
        assert_eq!(Square::F3.get_file(), 5);
        assert_eq!(Square::G6.get_file(), 6);
        assert_eq!(Square::H8.get_file(), 7);
    }
}
