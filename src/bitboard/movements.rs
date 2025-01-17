use crate::bitboard::BitBoard;

use super::sliding_pieces_with_hq::{get_bishop_attacks, get_queen_attacks, get_rook_attacks};

impl BitBoard {
    pub fn get_king_moves(self, own_pieces: BitBoard) -> BitBoard {
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

    pub fn get_knight_moves(self, own_pieces: BitBoard) -> BitBoard {
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

    pub fn get_white_pawn_attacks(self, all_other_pieces: BitBoard) -> BitBoard {
        // Left side of the pawn, minding the underflow File A.
        let pawn_left_attack = (self & BitBoard::NOT_A_FILE) << 7;
        // Right side
        let pawn_right_attack = (self & BitBoard::NOT_H_FILE) << 9;
        let pawn_attacks = pawn_left_attack | pawn_right_attack;

        // Is there something to attack?
        pawn_attacks & all_other_pieces
    }

    pub fn get_white_pawn_moves(
        self,
        all_pieces: BitBoard,
        all_other_pieces: BitBoard,
    ) -> BitBoard {
        // Pawns move in different ways for each color, so we need to seperate functions to
        // deal with the change in shifting and the opponents color.

        // Check the single space in front of the white pawn.
        let pawn_one_step = (self << 8) & !all_pieces;

        // For all moves that came from rank 2 (home row) and passed the above filter,
        // thereby being on rank 3, check and see if I can move forward one more.
        let pawn_two_steps = ((pawn_one_step & BitBoard::MASK_RANK_3) << 8) & !all_pieces;

        // The union of the movements dictate the possible moves forward available.
        let pawn_valid_moves = pawn_one_step | pawn_two_steps;

        // Pawn attacks:
        // Left side of the pawn, minding the underflow File A.
        // let pawn_left_attack = (self & BitBoard::NOT_A_FILE) << 7;
        // // Right side
        // let pawn_right_attack = (self & BitBoard::NOT_H_FILE) << 9;
        // let pawn_attacks = pawn_left_attack | pawn_right_attack;

        // Pawn attacks:
        let pawn_valid_attacks = self.get_white_pawn_attacks(all_other_pieces);

        pawn_valid_moves | pawn_valid_attacks
    }

    pub fn get_black_pawn_attacks(self, all_other_pieces: BitBoard) -> BitBoard {
        let pawn_left_attack = (self & BitBoard::NOT_A_FILE) >> 9;
        let pawn_right_attack = (self & BitBoard::NOT_H_FILE) >> 7;
        let pawn_attacks = pawn_left_attack | pawn_right_attack;

        pawn_attacks & all_other_pieces
    }

    pub fn get_black_pawn_moves(
        self,
        all_pieces: BitBoard,
        all_other_pieces: BitBoard,
    ) -> BitBoard {
        let pawn_one_step = (self >> 8) & !all_pieces;
        // For all moves that came from rank 7 (home row) and passed the above filter.
        let pawn_two_steps = ((pawn_one_step & BitBoard::MASK_RANK_6) >> 8) & !all_pieces;
        let pawn_valid_moves = pawn_one_step | pawn_two_steps;

        let pawn_valid_attacks = self.get_black_pawn_attacks(all_other_pieces);
        pawn_valid_moves | pawn_valid_attacks
    }

    pub fn get_bishop_moves(self, all_pieces: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let own: u64 = own_pieces.into();
        let val = get_bishop_attacks(all_pieces.into(), self.get_index()) & !own;
        BitBoard::new(val)
    }

    pub fn get_rook_moves(self, all_pieces: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let own: u64 = own_pieces.into();
        let val = get_rook_attacks(all_pieces.into(), self.get_index()) & !own;
        BitBoard::new(val)
    }

    pub fn get_queen_moves(self, all_pieces: BitBoard, own_pieces: BitBoard) -> BitBoard {
        let own: u64 = own_pieces.into();
        let val = get_queen_attacks(all_pieces.into(), self.get_index()) & !own;
        BitBoard::new(val)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Square::*;

    use super::*;

    #[test]
    fn test_king_moves_empty_board() {
        let king: BitBoard = E1.into();
        let moves = king.get_king_moves(BitBoard::EMPTY);
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
        let moves = king.get_king_moves(BitBoard::EMPTY);
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
        let moves = king.get_king_moves(BitBoard::EMPTY);
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
        let moves = king.get_king_moves(own_pieces);
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
        let moves = knight.get_knight_moves(own_pieces);
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

    #[test]
    fn test_white_pawn_moves() {
        let pawns: BitBoard = r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 1 0
            1 1 1 1 1 1 1 1
            0 0 0 0 0 0 0 0"
            .into();
        let all_pieces: BitBoard = r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 1 0 0 0
            1 0 1 1 0 0 1 0
            1 1 1 1 1 1 1 1
            0 0 0 0 0 0 0 0"
            .into();
        let all_black_pieces: BitBoard = r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            1 0 1 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0"
            .into();
        let moves = pawns.get_white_pawn_moves(all_pieces, all_black_pieces);
        assert_eq!(
            moves,
            r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 1 0 0 0 1 1 1
            1 1 1 0 1 1 0 1
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0"
                .into()
        );
    }
}
