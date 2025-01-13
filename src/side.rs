#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

impl Side {
    pub fn new(v: usize) -> Self {
        match v {
            0 => Self::White,
            1 => Self::Black,
            _ => panic!("Invalid side value"),
        }
    }
}
