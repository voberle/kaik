//! Parsing and creation of FEN strings.
//! Doc: <https://www.chessprogramming.org/Forsyth-Edwards_Notation>

use itertools::Itertools;

use crate::pieces::{Piece, PieceListBoard};
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

fn parse_piece_placement(s: &str) -> PieceListBoard {
    let pieces = s
        .split('/')
        .flat_map(|rank| {
            rank.chars().flat_map(|c| {
                if let Some(d) = c.to_digit(10) {
                    assert!((1..=8).contains(&d));
                    vec![None; d as usize]
                } else {
                    vec![c.try_into().ok()]
                }
            })
        })
        .collect_vec();
    assert_eq!(pieces.len(), 64);
    pieces
}

fn parse_side_to_move(s: &str) -> Side {
    match s {
        "w" => Side::White,
        "b" => Side::Black,
        _ => panic!("Invalid side to move"),
    }
}

fn parse_castling_ability(s: &str) -> Vec<Piece> {
    if s == "-" {
        Vec::new()
    } else {
        s.chars().map(|c| c.try_into().unwrap()).collect()
    }
}

fn parse_en_passant_target_square(s: &str) -> Option<Square> {
    if s == "-" {
        None
    } else {
        s.try_into().ok()
    }
}

fn parse_half_move_clock(s: &str) -> usize {
    s.parse().unwrap()
}

fn parse_full_move_counter(s: &str) -> usize {
    s.parse().unwrap()
}

pub fn parse(
    fen: &str,
) -> (
    PieceListBoard,
    Side,
    Vec<Piece>,
    Option<Square>,
    usize,
    usize,
) {
    let parts = fen.split_ascii_whitespace().collect_vec();
    assert_eq!(parts.len(), 6);
    (
        parse_piece_placement(parts[0]),
        parse_side_to_move(parts[1]),
        parse_castling_ability(parts[2]),
        parse_en_passant_target_square(parts[3]),
        parse_half_move_clock(parts[4]),
        parse_full_move_counter(parts[5]),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::{self, Piece::*};
    use crate::side::Side;
    use crate::squares::Square;

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

    #[test]
    fn test_parse_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let (pieces, side, castling, en_passant, half_move, full_move) = parse(fen);

        assert_eq!(pieces.len(), 64);
        assert_eq!(side, Side::White);
        assert_eq!(castling.len(), 4);
        assert!(castling.contains(&Piece::WhiteKing));
        assert!(castling.contains(&Piece::WhiteQueen));
        assert!(castling.contains(&Piece::BlackKing));
        assert!(castling.contains(&Piece::BlackQueen));
        assert_eq!(en_passant, None);
        assert_eq!(half_move, 0);
        assert_eq!(full_move, 1);
    }

    #[test]
    fn test_parse_middle_game_position() {
        let fen = "r1bqkbnr/pppppppp/2n5/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq e3 0 3";
        let (pieces, side, castling, en_passant, half_move, full_move) = parse(fen);

        assert_eq!(pieces.len(), 64);
        assert_eq!(side, Side::Black);
        assert_eq!(castling.len(), 4);
        assert!(castling.contains(&Piece::WhiteKing));
        assert!(castling.contains(&Piece::WhiteQueen));
        assert!(castling.contains(&Piece::BlackKing));
        assert!(castling.contains(&Piece::BlackQueen));
        assert_eq!(en_passant, Some(Square::try_from("e3").unwrap()));
        assert_eq!(half_move, 0);
        assert_eq!(full_move, 3);
    }

    #[test]
    fn test_parse_end_game_position() {
        let fen = "8/8/8/8/8/8/8/8 w - - 0 1";
        let (pieces, side, castling, en_passant, half_move, full_move) = parse(fen);

        assert_eq!(pieces.len(), 64);
        assert_eq!(side, Side::White);
        assert_eq!(castling.len(), 0);
        assert_eq!(en_passant, None);
        assert_eq!(half_move, 0);
        assert_eq!(full_move, 1);
    }

    #[test]
    fn test_parse_invalid_fen() {
        let fen = "invalid fen string";
        let result = std::panic::catch_unwind(|| parse(fen));
        assert!(result.is_err());
    }
}
