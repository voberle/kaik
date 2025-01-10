type BitBoard = u64;

pub fn print_bitboard(board: BitBoard) {
    for r in (0..8).rev() {
        for f in 0..8 {
            let index = r * 8 + f;
            let mask = 1 << index;
            // println!("r={r}, f={f}, index={index}, mask={mask:b}");
            let v = if board & mask == 0 { 0 } else { 1 };
            print!("{v}");
        }
        println!();
    }
}
