//! Parsing and creation of FEN strings.
//! Doc: <https://www.chessprogramming.org/Forsyth-Edwards_Notation>

use itertools::Itertools;

use crate::pieces::Piece;
use crate::side::Side;
use crate::squares::Square;

fn create_rank(rank: &[Option<Piece>]) -> String {
    assert_eq!(rank.len(), 8);
    let mut result = String::with_capacity(8);
    let mut empty_count = 0;
    for piece in rank {
        if let Some(p) = piece {
            if empty_count > 0 {
                result.push_str(&empty_count.to_string());
                empty_count = 0;
            }
            result.push(Into::<char>::into(*p));
        } else {
            empty_count += 1;
        }
    }
    if empty_count > 0 {
        result.push_str(&empty_count.to_string());
    }
    result
}

fn get_piece_placement(piece_placement: &[Option<Piece>]) -> String {
    assert_eq!(piece_placement.len(), 64);
    piece_placement.chunks(8).map(create_rank).join("/")
}

fn get_side_to_move(side_to_move: Side) -> &'static str {
    match side_to_move {
        Side::White => "w",
        Side::Black => "b",
    }
}

fn get_castling_ability(castling_ability: &[Piece]) -> String {
    if castling_ability.is_empty() {
        return "-".to_string();
    }

    assert!(castling_ability.len() <= 4);
    assert!([
        Piece::WhiteKing,
        Piece::WhiteQueen,
        Piece::BlackKing,
        Piece::BlackQueen
    ]
    .iter()
    .all(|piece| castling_ability.contains(piece)));

    castling_ability
        .iter()
        .map(|piece| Into::<char>::into(*piece))
        .join("")
}

fn get_en_passant_target_square(square: Option<Square>) -> String {
    if let Some(s) = square {
        let rank = s.get_rank();
        assert!([3, 6].contains(&rank));
        format!("{}{}", s.get_file(), rank)
    } else {
        "-".to_string()
    }
}

fn get_half_move_clock(half_move_clock: usize) -> String {
    half_move_clock.to_string()
}

fn get_full_move_counter(full_move_counter: usize) -> String {
    assert!(full_move_counter > 0);
    full_move_counter.to_string()
}

pub fn create(
    piece_placement: &[Option<Piece>],
    side_to_move: Side,
    castling_ability: &[Piece], // max 4, only king or queen
    en_passant_target_square: Option<Square>,
    half_move_clock: usize,
    full_move_counter: usize,
) -> String {
    format!(
        "{} {} {} {} {} {}",
        get_piece_placement(piece_placement),
        get_side_to_move(side_to_move),
        get_castling_ability(castling_ability),
        get_en_passant_target_square(en_passant_target_square),
        get_half_move_clock(half_move_clock),
        get_full_move_counter(full_move_counter),
    )
}

pub fn parse(fen: &str) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::{self, Piece::*};

    #[test]
    fn test_create_rank() {
        let rank = [
            Some(WhiteRook),
            Some(WhiteKnight),
            Some(WhiteBishop),
            Some(WhiteQueen),
            Some(WhiteKing),
            Some(WhiteBishop),
            Some(WhiteKnight),
            Some(WhiteRook),
        ];
        assert_eq!(create_rank(&rank), "RNBQKBNR");

        let rank = [None; 8];
        assert_eq!(create_rank(&rank), "8");

        let rank = [None, None, None, None, Some(WhitePawn), None, None, None];
        assert_eq!(create_rank(&rank), "4P3");

        let rank = [
            Some(WhiteRook),
            None,
            None,
            None,
            Some(WhitePawn),
            None,
            None,
            Some(BlackKing),
        ];
        assert_eq!(create_rank(&rank), "R3P2k");
    }

    #[test]
    fn test_create_rank_starting_position() {
        let piece_placement = pieces::parse(
            "rnbqkbnr pppppppp ........ ........ ........ ........ PPPPPPPP RNBQKBNR",
        );
        let castling_ability = [WhiteKing, WhiteQueen, BlackKing, BlackQueen];
        let fen = create(&piece_placement, Side::White, &castling_ability, None, 0, 1);
        assert_eq!(
            fen,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_create_rank_2nd_position() {
        let piece_placement = pieces::parse(
            "rnbqkbnr pp.ppppp ........ ..p..... ....P... ........ PPPP.PPP RNBQKBNR",
        );
        let castling_ability = [WhiteKing, WhiteQueen, BlackKing, BlackQueen];
        let fen = create(
            &piece_placement,
            Side::White,
            &castling_ability,
            Some(Square::C6),
            0,
            2,
        );
        assert_eq!(
            fen,
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2"
        );
    }
}
