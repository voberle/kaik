//! Generates attack bitboards for sliding pieces.
//! Hyperbola Quintessence approach.
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::unreadable_literal)]

use std::sync::OnceLock;

use itertools::Itertools;

// Masks for lines, ranks, diagonals.
// <https://www.chessprogramming.org/On_an_empty_Board#By_Calculation_3>

const fn rank_mask(sq: u64) -> u64 {
    0xFF << (sq & 56)
}

const fn file_mask(sq: u64) -> u64 {
    0x0101010101010101 << (sq & 7)
}

const fn diagonal_mask(sq: u64) -> u64 {
    const MAIN_DIAG: u64 = 0x8040201008040201;
    let diag: i8 = (sq & 7) as i8 - (sq >> 3) as i8;
    if diag >= 0 {
        MAIN_DIAG >> (diag * 8)
    } else {
        MAIN_DIAG << (-diag * 8)
    }
}

const fn anti_diagonal_mask(sq: u64) -> u64 {
    const MAIN_DIAG: u64 = 0x0102040810204080;
    let diag: i8 = 7 - (sq & 7) as i8 - (sq >> 3) as i8;
    if diag >= 0 {
        MAIN_DIAG >> (diag * 8)
    } else {
        MAIN_DIAG << (-diag * 8)
    }
}

// Excluding the square bit

const fn rank_mask_ex(sq: u64) -> u64 {
    (1 << sq) ^ rank_mask(sq)
}
const fn file_mask_ex(sq: u64) -> u64 {
    (1 << sq) ^ file_mask(sq)
}
const fn diagonal_mask_ex(sq: u64) -> u64 {
    (1 << sq) ^ diagonal_mask(sq)
}
const fn anti_diagonal_mask_ex(sq: u64) -> u64 {
    (1 << sq) ^ anti_diagonal_mask(sq)
}

// The masks for all 64 squares.
struct MaskForSquare {
    bit_mask: u64, // 1 << sq for convenience
    diagonal_mask_ex: u64,
    anti_diagonal_mask_ex: u64,
    file_mask_ex: u64,
}

impl MaskForSquare {
    fn new(sq: u64) -> Self {
        Self {
            bit_mask: 1 << sq,
            diagonal_mask_ex: diagonal_mask_ex(sq),
            anti_diagonal_mask_ex: anti_diagonal_mask_ex(sq),
            file_mask_ex: file_mask_ex(sq),
        }
    }
}

fn init_mask_for_square() -> [MaskForSquare; 64] {
    (0..64).map(MaskForSquare::new).collect_array().unwrap()
}

// Line masks for all squares, statically initialized.
fn get_masks(sq: u8) -> &'static MaskForSquare {
    static MASKS: OnceLock<[MaskForSquare; 64]> = OnceLock::new();
    &MASKS.get_or_init(init_mask_for_square)[sq as usize]
}

// For diagonals and file attacks, we just use hyperbola quintessence.
// <https://www.chessprogramming.org/Hyperbola_Quintessence>

fn line_attacks(occ: u64, sq: u8, mask: u64) -> u64 {
    let masks = get_masks(sq);
    let mut forward = occ & mask;
    let mut reverse = forward.swap_bytes();
    forward = forward.wrapping_sub(masks.bit_mask);
    reverse = reverse.wrapping_sub(masks.bit_mask.swap_bytes());
    forward ^= reverse.swap_bytes();
    forward &= mask;
    forward
}

fn diagonal_attacks(occ: u64, sq: u8) -> u64 {
    line_attacks(occ, sq, get_masks(sq).diagonal_mask_ex)
}

fn anti_diagonal_attacks(occ: u64, sq: u8) -> u64 {
    line_attacks(occ, sq, get_masks(sq).anti_diagonal_mask_ex)
}

fn file_attacks(occ: u64, sq: u8) -> u64 {
    line_attacks(occ, sq, get_masks(sq).file_mask_ex)
}

// For rank attacks it's more complicated.
// See <https://www.chessprogramming.org/First_Rank_Attacks#Attacks_on_all_Ranks>
// and explanations how it works
// <https://www.talkchess.com/forum3/viewtopic.php?t=71312&start=10>

// Generate an attack mask containing empty squares and the first occupied square.
fn generate_rank_attack_mask(occ: u64, file: u64) -> u64 {
    let mut mask = 0;
    for x in (0..file).rev() {
        let b = 1 << x;
        mask |= b;
        if (occ & b) == b {
            break;
        }
    }
    for x in (file + 1)..8 {
        let b = 1 << x;
        mask |= b;
        if (occ & b) == b {
            break;
        }
    }
    mask
}

fn init_rank_attack_mask_array() -> [u64; 512] {
    (0..64)
        .flat_map(|x| (0..8).map(move |file| generate_rank_attack_mask(x * 2, file)))
        .collect_array()
        .unwrap()
}

// Line masks for all squares, statically initialized.
fn get_rank_attack_mask(i: u64) -> u64 {
    static MASKS: OnceLock<[u64; 512]> = OnceLock::new();
    MASKS.get_or_init(init_rank_attack_mask_array)[i as usize]
}

fn rank_attacks(occ: u64, sq: u8) -> u64 {
    let file = u64::from(sq & 7);
    let rkx8 = sq & 56; // rank * 8
    let rank_occ_x2 = (occ >> rkx8) & (2 * 63); // 2 times the inner six bit rank occupancy used as index
    let attacks = get_rank_attack_mask(4 * rank_occ_x2 + file); // 8 * rank occupancy + file
    attacks << rkx8
}

pub fn get_rook_attacks(occ: u64, sq: u8) -> u64 {
    file_attacks(occ, sq) | rank_attacks(occ, sq)
}

pub fn get_bishop_attacks(occ: u64, sq: u8) -> u64 {
    diagonal_attacks(occ, sq) | anti_diagonal_attacks(occ, sq)
}

pub fn get_queen_attacks(occ: u64, sq: u8) -> u64 {
    get_rook_attacks(occ, sq) | get_bishop_attacks(occ, sq)
}

#[cfg(test)]
mod tests {
    use crate::bitboard::BitBoard;

    use super::*;

    #[test]
    fn test_masks() {
        const C5: u64 = 34;
        assert_eq!(rank_mask(C5), 1095216660480);
        assert_eq!(file_mask(C5), 289360691352306692);
        assert_eq!(diagonal_mask(C5), 2310355422147575808);
        assert_eq!(anti_diagonal_mask(C5), 283691315109952);
    }

    #[test]
    fn test_bishop_attacks() {
        const C5: u8 = 34;
        let occupancy: BitBoard = r"
            . . . . . 1 . .
            . . . . 1 . . .
            . 1 . . . . . .
            . . 1 . . . . .
            . . . . . . . .
            . . . . 1 . 1 .
            1 1 1 1 1 . 1 1
            . . . . . . 1 ."
            .into();
        let attacks = get_bishop_attacks(occupancy.into(), C5);
        assert_eq!(
            BitBoard::new(attacks),
            r"
            . . . . . . . .
            . . . . 1 . . .
            . 1 . 1 . . . .
            . . . . . . . .
            . 1 . 1 . . . .
            1 . . . 1 . . .
            . . . . . . . .
            . . . . . . . .
            "
            .into()
        );
    }

    #[test]
    fn test_rook_attacks() {
        const C5: u8 = 34;
        let occupancy: BitBoard = r"
            . . . . . 1 . .
            . . . . 1 . . .
            . 1 . . . . . .
            . . 1 . . 1 . .
            . . . . . . . .
            . . . . 1 . 1 .
            1 1 1 1 1 . 1 1
            . . . . . . 1 ."
            .into();
        occupancy.print();
        let attacks = get_rook_attacks(occupancy.into(), C5);
        BitBoard::new(attacks).print();
        assert_eq!(
            BitBoard::new(attacks),
            r"
            . . 1 . . . . .
            . . 1 . . . . .
            . . 1 . . . . .
            1 1 . 1 1 1 . .
            . . 1 . . . . .
            . . 1 . . . . .
            . . 1 . . . . .
            . . . . . . . .
            "
            .into()
        );
    }
}
