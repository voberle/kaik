use crate::bitboard::BitBoard;

impl BitBoard {
    pub fn get_king_attacks(self, own_pieces: BitBoard) -> BitBoard {
        // See Peter Keller https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/index.html
        // NB: The code there is buggy...
        // 1 2 3    +7 +8 +9
        // 8 K 4    -1  K +1
        // 7 6 5    -9 -8 -7

        // Ignore the rank clipping since the overflow/underflow simply vanishes. We only care about the file overflow/underflow.
        let king_clip_file_h = self & BitBoard::NOT_H_FILE;
        let king_clip_file_a = self & BitBoard::NOT_A_FILE;

        let spot_1 = king_clip_file_a << 7;
        let spot_2 = self << 8;
        let spot_3 = king_clip_file_h << 9;
        let spot_4 = king_clip_file_h << 1;

        let spot_5 = king_clip_file_h >> 7;
        let spot_6 = self >> 8;
        let spot_7 = king_clip_file_a >> 9;
        let spot_8 = king_clip_file_a >> 1;

        let moves = spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8;

        moves & !own_pieces
    }

    pub fn get_knight_attacks(self, own_pieces: BitBoard) -> BitBoard {
        //  2 3
        // 1   3
        //   N
        // 8   5
        //  7 6
        let knight_clip_file_ab = self & BitBoard::NOT_AB_FILE;
        let knight_clip_file_a = self & BitBoard::NOT_A_FILE;
        let knight_clip_file_h = self & BitBoard::NOT_H_FILE;
        let knight_clip_file_hg = self & BitBoard::NOT_HG_FILE;

        let spot_1 = knight_clip_file_ab << 6;
        let spot_2 = knight_clip_file_a << 15;
        let spot_3 = knight_clip_file_h << 17;
        let spot_4 = knight_clip_file_hg << 10;

        let spot_5 = knight_clip_file_hg >> 6;
        let spot_6 = knight_clip_file_h >> 15;
        let spot_7 = knight_clip_file_a >> 17;
        let spot_8 = knight_clip_file_ab >> 10;

        let moves = spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8;

        moves & !own_pieces
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Square::*;

    use super::*;

    #[test]
    fn test_king_moves_empty_board() {
        let king: BitBoard = E1.into();
        let moves = king.get_king_attacks(BitBoard::EMPTY);
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
        let moves = king.get_king_attacks(BitBoard::EMPTY);
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
        let moves = king.get_king_attacks(BitBoard::EMPTY);
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
        let moves = king.get_king_attacks(own_pieces);
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

    #[test]
    fn test_knight_moves() {
        let knight: BitBoard = B4.into();
        let own_pieces: BitBoard = Into::<BitBoard>::into(D4) | A2.into() | D1.into();
        let moves = knight.get_knight_attacks(own_pieces);
        assert_eq!(
            moves,
            r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            1 0 1 0 0 0 0 0
            0 0 0 1 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 1 0 0 0 0
            0 0 1 0 0 0 0 0
            0 0 0 0 0 0 0 0"
                .into()
        );
    }
}
