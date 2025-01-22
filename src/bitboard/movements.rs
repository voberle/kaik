use crate::bitboard;
use crate::bitboard::BitBoard;

use super::{
    constants::{MASK_RANK_3, MASK_RANK_6, NOT_AB_FILE, NOT_A_FILE, NOT_HG_FILE, NOT_H_FILE},
    sliding_pieces_with_hq,
};

pub fn get_king_attacks(king_pos: BitBoard) -> BitBoard {
    // See Peter Keller https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/index.html
    // NB: The code there is buggy...
    // 1 2 3    +7 +8 +9
    // 8 K 4    -1  K +1
    // 7 6 5    -9 -8 -7

    // Ignore the rank clipping since the overflow/underflow simply vanishes. We only care about the file overflow/underflow.
    let king_clip_file_h = king_pos & NOT_H_FILE;
    let king_clip_file_a = king_pos & NOT_A_FILE;

    let spot_1 = king_clip_file_a << 7;
    let spot_2 = king_pos << 8;
    let spot_3 = king_clip_file_h << 9;
    let spot_4 = king_clip_file_h << 1;

    let spot_5 = king_clip_file_h >> 7;
    let spot_6 = king_pos >> 8;
    let spot_7 = king_clip_file_a >> 9;
    let spot_8 = king_clip_file_a >> 1;

    spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8
}

pub fn get_king_moves(king_pos: BitBoard, own_pieces: BitBoard) -> BitBoard {
    get_king_attacks(king_pos) & !own_pieces
}

pub fn get_knight_attacks(knights_pos: BitBoard) -> BitBoard {
    //  2 3
    // 1   3
    //   N
    // 8   5
    //  7 6
    let knight_clip_file_ab = knights_pos & NOT_AB_FILE;
    let knight_clip_file_a = knights_pos & NOT_A_FILE;
    let knight_clip_file_h = knights_pos & NOT_H_FILE;
    let knight_clip_file_hg = knights_pos & NOT_HG_FILE;

    let spot_1 = knight_clip_file_ab << 6;
    let spot_2 = knight_clip_file_a << 15;
    let spot_3 = knight_clip_file_h << 17;
    let spot_4 = knight_clip_file_hg << 10;

    let spot_5 = knight_clip_file_hg >> 6;
    let spot_6 = knight_clip_file_h >> 15;
    let spot_7 = knight_clip_file_a >> 17;
    let spot_8 = knight_clip_file_ab >> 10;

    spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8
}

pub fn get_knight_moves(knights_pos: BitBoard, own_pieces: BitBoard) -> BitBoard {
    get_knight_attacks(knights_pos) & !own_pieces
}

pub fn get_white_pawn_attacks(pawns_pos: BitBoard) -> BitBoard {
    // Left side of the pawn, minding the underflow File A.
    let pawn_left_attack = (pawns_pos & NOT_A_FILE) << 7;
    // Right side
    let pawn_right_attack = (pawns_pos & NOT_H_FILE) << 9;
    pawn_left_attack | pawn_right_attack
}

pub fn get_valid_white_pawn_attacks(pawns_pos: BitBoard, all_other_pieces: BitBoard) -> BitBoard {
    // Is there something to attack?
    get_white_pawn_attacks(pawns_pos) & all_other_pieces
}

pub fn get_white_pawn_moves(
    pawns_pos: BitBoard,
    all_pieces: BitBoard,
    all_other_pieces: BitBoard,
) -> BitBoard {
    // Pawns move in different ways for each color, so we need to seperate functions to
    // deal with the change in shifting and the opponents color.

    // Check the single space in front of the white pawn.
    let pawn_one_step = (pawns_pos << 8) & !all_pieces;

    // For all moves that came from rank 2 (home row) and passed the above filter,
    // thereby being on rank 3, check and see if I can move forward one more.
    let pawn_two_steps = ((pawn_one_step & MASK_RANK_3) << 8) & !all_pieces;

    // The union of the movements dictate the possible moves forward available.
    let pawn_valid_moves = pawn_one_step | pawn_two_steps;

    // Pawn attacks:
    let pawn_valid_attacks = get_valid_white_pawn_attacks(pawns_pos, all_other_pieces);

    pawn_valid_moves | pawn_valid_attacks
}

pub fn get_black_pawn_attacks(pawns_pos: BitBoard) -> BitBoard {
    let pawn_left_attack = (pawns_pos & NOT_A_FILE) >> 9;
    let pawn_right_attack = (pawns_pos & NOT_H_FILE) >> 7;
    pawn_left_attack | pawn_right_attack
}

pub fn get_valid_black_pawn_attacks(pawns_pos: BitBoard, all_other_pieces: BitBoard) -> BitBoard {
    get_black_pawn_attacks(pawns_pos) & all_other_pieces
}

pub fn get_black_pawn_moves(
    pawns_pos: BitBoard,
    all_pieces: BitBoard,
    all_other_pieces: BitBoard,
) -> BitBoard {
    let pawn_one_step = (pawns_pos >> 8) & !all_pieces;
    // For all moves that came from rank 7 (home row) and passed the above filter.
    let pawn_two_steps = ((pawn_one_step & MASK_RANK_6) >> 8) & !all_pieces;
    let pawn_valid_moves = pawn_one_step | pawn_two_steps;

    let pawn_valid_attacks = get_valid_black_pawn_attacks(pawns_pos, all_other_pieces);
    pawn_valid_moves | pawn_valid_attacks
}

pub fn get_bishop_attacks(bishops_pos: BitBoard, all_pieces: BitBoard) -> BitBoard {
    BitBoard::new(sliding_pieces_with_hq::get_bishop_attacks(
        all_pieces.into(),
        bitboard::get_index(bishops_pos),
    ))
}

pub fn get_rook_attacks(rooks_pos: BitBoard, all_pieces: BitBoard) -> BitBoard {
    BitBoard::new(sliding_pieces_with_hq::get_rook_attacks(
        all_pieces.into(),
        bitboard::get_index(rooks_pos),
    ))
}

pub fn get_bishop_moves(
    bishops_pos: BitBoard,
    all_pieces: BitBoard,
    own_pieces: BitBoard,
) -> BitBoard {
    let own: u64 = own_pieces.into();
    let val = sliding_pieces_with_hq::get_bishop_attacks(
        all_pieces.into(),
        bitboard::get_index(bishops_pos),
    ) & !own;
    BitBoard::new(val)
}

pub fn get_rook_moves(rooks_pos: BitBoard, all_pieces: BitBoard, own_pieces: BitBoard) -> BitBoard {
    let own: u64 = own_pieces.into();
    let val =
        sliding_pieces_with_hq::get_rook_attacks(all_pieces.into(), bitboard::get_index(rooks_pos))
            & !own;
    BitBoard::new(val)
}

pub fn get_queen_moves(
    queens_pos: BitBoard,
    all_pieces: BitBoard,
    own_pieces: BitBoard,
) -> BitBoard {
    let own: u64 = own_pieces.into();
    let val = sliding_pieces_with_hq::get_queen_attacks(
        all_pieces.into(),
        bitboard::get_index(queens_pos),
    ) & !own;
    BitBoard::new(val)
}

#[cfg(test)]
mod tests {
    use crate::{
        bitboard::{self, constants::EMPTY},
        common::Square::*,
    };

    use super::*;

    #[test]
    fn test_king_moves_empty_board() {
        let king: BitBoard = bitboard::from_square(E1);
        let moves = get_king_moves(king, EMPTY);
        assert_eq!(
            moves,
            bitboard::from_str(
                r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 1 1 1 0 0
            0 0 0 1 0 1 0 0"
            )
        );

        let king: BitBoard = bitboard::from_square(H4);
        let moves = get_king_moves(king, EMPTY);
        assert_eq!(
            moves,
            bitboard::from_str(
                r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 1 1
            0 0 0 0 0 0 1 0
            0 0 0 0 0 0 1 1
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0"
            )
        );

        let king: BitBoard = bitboard::from_square(A8);
        let moves = get_king_moves(king, EMPTY);
        assert_eq!(
            moves,
            bitboard::from_str(
                r"
            0 1 0 0 0 0 0 0
            1 1 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0"
            )
        );
    }

    #[test]
    fn test_king_moves_not_empty_board() {
        let king: BitBoard = bitboard::from_square(E1);
        let own_pieces: BitBoard = bitboard::from_square(D2) | bitboard::from_square(F1);
        let moves = get_king_moves(king, own_pieces);
        assert_eq!(
            moves,
            bitboard::from_str(
                r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 1 1 0 0
            0 0 0 1 0 0 0 0"
            )
        );
    }

    #[test]
    fn test_knight_moves() {
        let knight: BitBoard = bitboard::from_square(B4);
        let own_pieces: BitBoard =
            bitboard::from_square(D4) | bitboard::from_square(A2) | bitboard::from_square(D1);
        let moves = get_knight_moves(knight, own_pieces);
        assert_eq!(
            moves,
            bitboard::from_str(
                r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            1 0 1 0 0 0 0 0
            0 0 0 1 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 1 0 0 0 0
            0 0 1 0 0 0 0 0
            0 0 0 0 0 0 0 0"
            )
        );
    }

    #[test]
    fn test_white_pawn_moves() {
        let pawns: BitBoard = bitboard::from_str(
            r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 1 0
            1 1 1 1 1 1 1 1
            0 0 0 0 0 0 0 0",
        );
        let all_pieces: BitBoard = bitboard::from_str(
            r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 1 0 0 0
            1 0 1 1 0 0 1 0
            1 1 1 1 1 1 1 1
            0 0 0 0 0 0 0 0",
        );
        let all_black_pieces: BitBoard = bitboard::from_str(
            r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            1 0 1 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0",
        );
        let moves = get_white_pawn_moves(pawns, all_pieces, all_black_pieces);
        assert_eq!(
            moves,
            bitboard::from_str(
                r"
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0
            0 1 0 0 0 1 1 1
            1 1 1 0 1 1 0 1
            0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0"
            )
        );
    }
}
