#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub const fn new(v: usize) -> Self {
        match v {
            0 => Self::White,
            1 => Self::Black,
            _ => panic!("Invalid side value"),
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
