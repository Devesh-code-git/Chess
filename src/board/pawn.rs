use crate::board::*;
use crate::chessmoves::*;

// Checks if an en_passant can be performed
pub fn en_passant(squares: &[Option<Piece>; 64], to: &ChessMove, from: &ChessMove, turn: &PieceColor) -> bool {
    if to.y - from.y != 2 && to.y - from.y != -2 {
        return false;
    }

    let mut left_square: Option<Piece> = None;
    let mut right_square: Option<Piece> = None;

    if to.x - 1 >= 0 {
        left_square = squares[ChessMove::new(to.x - 1, to.y).convert()]; 
    }

    if to.x + 1 <= 7 {
        right_square = squares[ChessMove::new(to.x + 1, to.y).convert()];
    }

    match left_square {
        None => {},
        Some(t) => {
            if t.piece == PieceType::PAWN && t.color == *turn {
                return true;
            }
        },
    };

    match right_square {
        None => {},
        Some(t) => {
            if t.piece == PieceType::PAWN && t.color == *turn {
                return true;
            }
        },
    };

    false
}

// Checks if an en_passant move has happened, if so then removes en_passant pawn
pub fn do_en_passant(squares: &mut [Option<Piece>; 64], file: &i32, rank: &i32, kind: &Piece, en_passant_idx: &Option<ChessMove>) {
    if kind.color == PieceColor::WHITE && kind.piece == PieceType::PAWN && en_passant_idx.is_some() {
        let en_passant = match en_passant_idx {
            None => return,
            Some(t) => t,
        };

        if *file == en_passant.x && *rank == en_passant.y - 1 {
            squares[en_passant.convert()] = None;
        }
    }
    else if kind.color == PieceColor::BLACK && kind.piece == PieceType::PAWN && en_passant_idx.is_some() {
        let en_passant = match en_passant_idx {
            None => return,
            Some(t) => t,
        };

        if *file == en_passant.x && *rank == en_passant.y + 1 {
            squares[en_passant.convert()] = None;
        }
    }
}

// Checks if player performed a pawn_promotion
pub fn pawn_promotion(pawn_promotion_idx: &mut Option<ChessMove>, file: &i32, rank: &i32, kind: &Piece) {
    if kind.piece == PieceType::PAWN {
        if kind.color == PieceColor::WHITE && *rank == 0 {
            *pawn_promotion_idx = Some(ChessMove::new(*file, *rank));
        }
        else if kind.color == PieceColor::BLACK && *rank == 7 {
            *pawn_promotion_idx = Some(ChessMove::new(*file, *rank));
        }
    }
}
