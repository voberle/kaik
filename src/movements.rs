use crate::{
    bitboard::BitBoard,
    constants::{NOT_A_FILE, NOT_H_FILE},
};

fn king_moves(king_loc: BitBoard, own_pieces: BitBoard) -> BitBoard {
    // See Peter Keller https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/index.html
    // NB: The code there is buggy...
    // 1 2 3    +7 +8 +9
    // 8 K 4    -1  K +1
    // 7 6 5    -9 -8 -7

    // Ignore the rank clipping since the overflow/underflow simply vanishes. We only care about the file overflow/underflow.
    let king_clip_file_h = king_loc & NOT_H_FILE;
    let king_clip_file_a = king_loc & NOT_A_FILE;

    let spot_1 = king_clip_file_a << 7;
    let spot_2 = king_loc << 8;
    let spot_3 = king_clip_file_h << 9;
    let spot_4 = king_clip_file_h << 1;
    let spot_5 = king_clip_file_h >> 7;
    let spot_6 = king_loc >> 8;
    let spot_7 = king_clip_file_a >> 9;
    let spot_8 = king_clip_file_a >> 1;

    let moves = spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8;

    moves & !own_pieces
}

#[cfg(test)]
mod tests {
    use crate::squares::Square::*;

    use super::*;

    #[test]
    fn test_king_moves_empty_board() {
        let king: BitBoard = E1.into();
        let moves = king_moves(king, BitBoard::EMPTY);
        assert_eq!(
            moves,
            r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 1 1 1 0 0
            0 0 0 1 0 1 0 0"
                .into()
        );

        let king: BitBoard = H4.into();
        let moves = king_moves(king, BitBoard::EMPTY);
        assert_eq!(
            moves,
            r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 1 1
            0 0 0 0 0 0 1 0
            0 0 0 0 0 0 1 1
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0"
                .into()
        );

        let king: BitBoard = A8.into();
        let moves = king_moves(king, BitBoard::EMPTY);
        assert_eq!(
            moves,
            r"
            0 1 0 0 0 0 0 0
            1 1 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0"
                .into()
        );
    }

    #[test]
    fn test_king_moves_not_empty_board() {
        let king: BitBoard = E1.into();
        let own_pieces: BitBoard = Into::<BitBoard>::into(D2) | F1.into();
        own_pieces.print();
        let moves = king_moves(king, own_pieces);
        moves.print();
        assert_eq!(
            moves,
            r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 1 1 0 0
            0 0 0 1 0 0 0 0"
                .into()
        );
    }
}
