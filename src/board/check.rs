use crate::chessmoves::*;
use crate::board::*;

// Function checks if current turns king is in check by seeing from the kings position if any
// opppsite colored piece attacks it
pub fn is_check(squares: &[Option<Piece>; 64], turn: PieceColor, wking_idx: &ChessMove, bking_idx: &ChessMove) -> (bool, f32, f32) {
    let file;
    let rank;

    if turn == PieceColor::WHITE {
        file = wking_idx.x;
        rank = wking_idx.y;
    }
    else {
        file = bking_idx.x;
        rank = bking_idx.y;
    }

    if does_pawn_attack(squares, turn, file, rank) || does_knight_attack(squares, turn, file, rank) || does_king_attack(squares, turn, file, rank) || 
        does_rook_or_queen_attack(squares, turn, file, rank) || does_bishop_or_queen_attack(squares, turn, file, rank) 
    {
        return (true, file as f32, rank as f32);
    }

    (false, 0.0, 0.0)
}

// Checks if from kings position and knights attack it
fn does_knight_attack(squares: &[Option<Piece>; 64], turn: PieceColor, file: i32, rank: i32) -> bool {
    if (file + 1 <= 7) && (rank - 2 >= 0) {
        match squares[ChessMove::new(file + 1, rank - 2).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KNIGHT && t.color != turn { return true; },
        };
    }

    if (file + 2 <= 7) && (rank - 1 >= 0) {
        match squares[ChessMove::new(file + 2, rank - 1).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KNIGHT && t.color != turn { return true; },
        };
    }

    if (file - 1 >= 0) && (rank - 2 >= 0) {
        match squares[ChessMove::new(file - 1, rank - 2).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KNIGHT && t.color != turn { return true; },
        };
    }

    if (file - 2 >= 0) && (rank - 1 >= 0) {
        match squares[ChessMove::new(file - 2, rank - 1).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KNIGHT && t.color != turn { return true; },
        };
    }

    if (file + 2 <= 7) && (rank + 1 <= 7) {
        match squares[ChessMove::new(file + 2, rank + 1).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KNIGHT && t.color != turn { return true; },
        };
    }

    if (file + 1 <= 7) && (rank + 2 <= 7) {
        match squares[ChessMove::new(file + 1, rank + 2).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KNIGHT && t.color != turn { return true; },
        };
    }

    if (file - 2 >= 0) && (rank + 1 <= 7) {
        match squares[ChessMove::new(file - 2, rank + 1).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KNIGHT && t.color != turn { return true; },
        };
    }

    if (file - 1 >= 0) && (rank + 2 <= 7) {
        match squares[ChessMove::new(file - 1, rank + 2).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KNIGHT && t.color != turn { return true; },
        };
    }

    false
}

// Checks from current kings position if the other king attacks it
fn does_king_attack(squares: &[Option<Piece>; 64], turn: PieceColor, file: i32, rank: i32) -> bool {
    if rank - 1 >= 0 {
        match squares[ChessMove::new(file, rank - 1).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KING && t.color != turn { return true; },
        };
    }

    if (file - 1 >= 0) && (rank - 1 >= 0) {
        match squares[ChessMove::new(file - 1, rank - 1).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KING && t.color != turn { return true; },
        };
    }

    if (file + 1 <= 7) && (rank - 1 >= 0) {
        match squares[ChessMove::new(file + 1, rank - 1).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KING && t.color != turn { return true; },
        };
    }

    if file + 1 <= 7 {
        match squares[ChessMove::new(file + 1, rank).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KING && t.color != turn { return true; },
        };
    }

    if file - 1 >= 0 {
        match squares[ChessMove::new(file - 1, rank).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KING && t.color != turn { return true; },
        };
    }

    if rank + 1 <= 7 {
        match squares[ChessMove::new(file, rank + 1).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KING && t.color != turn { return true; },
        };
    }

    if (file + 1 <= 7) && (rank + 1 <= 7) {
        match squares[ChessMove::new(file + 1, rank + 1).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KING && t.color != turn { return true; },
        };
    }

    if (file - 1 >= 0) && (rank + 1 <= 7) {
        match squares[ChessMove::new(file - 1, rank + 1).convert()] {
            None => {},
            Some(t) => if t.piece == PieceType::KING && t.color != turn { return true; },
        };
    }

    false
}

// Checks from kings position, if and pawns attack it
fn does_pawn_attack(squares: &[Option<Piece>; 64], turn: PieceColor, file: i32, rank: i32) -> bool {
    if turn == PieceColor::WHITE {
        if (file - 1 >= 0) && (rank - 1 >= 0) {
            match squares[ChessMove::new(file - 1, rank - 1).convert()] {
                None => {},
                Some(t) => if t.piece == PieceType::PAWN && t.color != turn { return true; },
            };
        }

        if (file + 1 <= 7) && (rank - 1 >= 0) {
            match squares[ChessMove::new(file + 1, rank - 1).convert()] {
                None => {},
                Some(t) => if t.piece == PieceType::PAWN && t.color != turn { return true; },
            };
        }
    }

    if turn == PieceColor::BLACK {
        if (file - 1 >= 0) && (rank + 1 <= 7) {
            match squares[ChessMove::new(file - 1, rank + 1).convert()] {
                None => {},
                Some(t) => if t.piece == PieceType::PAWN && t.color != turn { return true; },
            };
        }

        if (file + 1 <= 7) && (rank + 1 <= 7) {
            match squares[ChessMove::new(file + 1, rank + 1).convert()] {
                None => {},
                Some(t) => if t.piece == PieceType::PAWN && t.color != turn { return true; },
            };
        }
    }

    false
}

// Checks from current kings position, if any rooks or queens attack it
fn does_rook_or_queen_attack(squares: &[Option<Piece>; 64], turn: PieceColor, file: i32, rank: i32) -> bool {
    // UP
    for i in 1..8 {
        if rank + i > 7  { break; }

        match squares[ChessMove::new(file, rank + i).convert()] {
            None => {},
            Some(t) => {
                if (t.piece == PieceType::ROOK || t.piece == PieceType::QUEEN) && t.color != turn { return true; }
                break;
            },
        };
    }

    // DOWN
    for i in 1..8 {
        if rank - i < 0 { break; }

        match squares[ChessMove::new(file, rank - i).convert()] {
            None => {},
            Some(t) => {
                if (t.piece == PieceType::ROOK || t.piece == PieceType::QUEEN) && t.color != turn { return true; }
                break;
            },
        };
    }

    // RIGHT
    for i in 1..8 {
        if file + i > 7 { break; }

        match squares[ChessMove::new(file + i, rank).convert()] {
            None => {},
            Some(t) => {
                if (t.piece == PieceType::ROOK || t.piece == PieceType::QUEEN) && t.color != turn { return true; }
                break;
            },
        };
    }


    // LEFT
    for i in 1..8 {
        if file - i < 0 { break; }

        match squares[ChessMove::new(file - i, rank).convert()] {
            None => {},
            Some(t) => {
                if (t.piece == PieceType::ROOK || t.piece == PieceType::QUEEN) && t.color != turn { return true; }
                break;
            },
        };
    }

    false
}

// Checks from current kings positoin is any bishops or queens attakc it
fn does_bishop_or_queen_attack(squares: &[Option<Piece>; 64], turn: PieceColor, file: i32, rank: i32) -> bool {
    // UP-RIGHT
    for i in 1..8 {
        if (file + i > 7) || (rank - i < 0) { break; }

        match squares[ChessMove::new(file + i, rank - i).convert()] {
            None => {},
            Some(t) => {
                if (t.piece == PieceType::BISHOP || t.piece == PieceType::QUEEN) && t.color != turn { return true; }
                break;
            },
        };
    }

    // UP-LEFT
    for i in 1..8 {
        if (file - i < 0) || (rank - i < 0) { break; }

        match squares[ChessMove::new(file - i, rank - i).convert()] {
            None => {},
            Some(t) => {
                if (t.piece == PieceType::BISHOP || t.piece == PieceType::QUEEN) && t.color != turn { return true; }
                break;
            },
        };
    }

    // DOWN-RIGHT
    for i in 1..8 {
        if (file + i > 7) || (rank + i > 7) { break; }

        match squares[ChessMove::new(file + i, rank + i).convert()] {
            None => {},
            Some(t) => {
                if (t.piece == PieceType::BISHOP || t.piece == PieceType::QUEEN) && t.color != turn { return true; }
                break;
            },
        };
    }

    // DOWN-LEFT
    for i in 1..8 {
        if (file - i < 0) || (rank + i > 7) { break; }

        match squares[ChessMove::new(file - i, rank + i).convert()] {
            None => {},
            Some(t) => {
                if (t.piece == PieceType::BISHOP || t.piece == PieceType::QUEEN) && t.color != turn { return true; }
                break;
            },
        };
    }

    false
}

pub fn is_checkmate_or_stalemate(squares: &mut [Option<Piece>; 64], turn: PieceColor, stalemate_idx: &mut Option<ChessMove>, is_check: bool, en_passant_idx: &Option<ChessMove>, wking_idx: &ChessMove, bking_idx: &ChessMove) -> (bool, bool) {
   for i in 0..64 {
       let piece_at = squares[i];

       match piece_at {
           None => {},
           Some(t) => {
               if t.color == turn {
                   let file = (i % 8) as i32;
                   let rank = (i / 8) as i32;
                   let from = ChessMove::new(file, rank);

                   if t.piece == PieceType::KING {*stalemate_idx = Some(from)}

                   let mut moves = from.get_legal_moves(&t, squares);
                   moves = filter_moves(squares, &moves, from, en_passant_idx, &turn, wking_idx, bking_idx);
                   if moves.len() > 0 {
                       return (false, false);
                   }
               }
           },
       };
   }

   if is_check {
       return (true, false); // Checkmate
   }
   else {
       return (false, true); // Stalemate
   }
}

// This helper function filters through the legal moves and keeps the ones that get the player
// out of check, or not in check
pub fn filter_moves(squares: &mut [Option<Piece>; 64], moves: &Vec<ChessMove>, from: ChessMove, en_passant_idx: &Option<ChessMove>, turn: &PieceColor, wking_idx: &ChessMove, bking_idx: &ChessMove) -> Vec<ChessMove> {
    let mut filterd_moves: Vec<ChessMove> = Vec::new();
    let piece_one = squares[from.convert()];
    let mut white_en_passant = false;
    let mut black_en_passant = false;
    let mut temporary_wking_idx = *wking_idx;
    let mut temporary_bking_idx = *bking_idx;

    for mv in moves {
        let piece_two = squares[mv.convert()];
        squares[mv.convert()] = piece_one;
        squares[from.convert()] = None;

        // If its an en_passant move, has to remove other pawn as well
        if en_passant_idx.is_some() && piece_one.is_some() {
            if piece_one.unwrap().piece == PieceType::PAWN {
                do_en_passant_move(squares, piece_one.unwrap(), mv, &mut white_en_passant, &mut black_en_passant, en_passant_idx.unwrap());
            }
        }

        // If we are simulating kings moves, have to update its position for the check function
        if piece_one.is_some() {
            let kind = piece_one.unwrap();

            if kind.piece == PieceType::KING && kind.color == PieceColor::WHITE { 
                temporary_wking_idx = *mv; 
            }
            else if kind.piece == PieceType::KING && kind.color == PieceColor::BLACK {
                temporary_bking_idx = *mv;
            }
        }

        if is_check(squares, *turn, &temporary_wking_idx, &temporary_bking_idx).0 == false {
            filterd_moves.push(*mv);
        }

        reverse_move(squares, *mv, from, piece_one, piece_two);

        // reverses an en_passant move if performed
        if white_en_passant {
            squares[en_passant_idx.unwrap().convert()] = Some(Piece::new(PieceType::PAWN, PieceColor::BLACK));
            white_en_passant = false;
        }
        else if black_en_passant {
            squares[en_passant_idx.unwrap().convert()] = Some(Piece::new(PieceType::PAWN, PieceColor::WHITE));
            black_en_passant = false;
        }
    }

    filterd_moves
}

// Simulates an en_passant move, where pawn moves and other pawn is taken
fn do_en_passant_move(squares: &mut [Option<Piece>; 64], piece_one: Piece, mv: &ChessMove, white_bool: &mut bool, black_bool: &mut bool, idx: ChessMove) {
    if piece_one.piece == PieceType::PAWN {
        if piece_one.color == PieceColor::WHITE && mv.x == idx.x && mv.y == idx.y - 1 {
            squares[idx.convert()] = None;
            *white_bool = true;
        }
        else if piece_one.color == PieceColor::BLACK && mv.x == idx.x && mv.y == idx.y + 1 {
            squares[idx.convert()] = None;
            *black_bool = true;
        }
    }
}

fn reverse_move(squares: &mut [Option<Piece>; 64], to: ChessMove, from: ChessMove, piece_one: Option<Piece>, piece_two: Option<Piece>) {
    squares[from.convert()] = piece_one;
    squares[to.convert()] = piece_two;
}
