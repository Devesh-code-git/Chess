use crate::board::*;
use crate::chessmoves::*;

pub struct Information {
    left_white_rook_moved: bool,
    right_white_rook_moved: bool,
    left_black_rook_moved: bool,
    right_black_rook_moved: bool,
    white_king_moved: bool,
    black_king_moved: bool,
    castle_moves: Vec<ChessMove>,
}

impl Information {
    pub fn new() -> Self {
        Self {
            left_white_rook_moved: false,
            right_white_rook_moved: false,
            left_black_rook_moved: false,
            right_black_rook_moved: false,
            white_king_moved: false,
            black_king_moved: false,
            castle_moves: Vec::new(),
        }
    }
}

impl Information {
    pub fn set_flags(&mut self, kind: &Piece, removed_piece: Option<Piece>, to: &ChessMove, from: &ChessMove, wking_idx: &mut ChessMove, bking_idx: &mut ChessMove) {
        match kind.color {
            PieceColor::WHITE => {
                match kind.piece {
                    PieceType::KING => {
                        self.white_king_moved = true;
                        *wking_idx = ChessMove::new(to.x, to.y);
                    },
                    PieceType::ROOK => if from.x == 0 { self.left_white_rook_moved = true } else { self.right_white_rook_moved = true },
                    _ => {},
                };
            },

            PieceColor::BLACK => {
                match kind.piece {
                    PieceType::KING => {
                        self.black_king_moved = true;
                        *bking_idx = ChessMove::new(to.x, to.y);
                    },
                    PieceType::ROOK => if from.x == 0 { self.left_black_rook_moved = true } else { self.right_black_rook_moved = true },
                    _ => {},
                };
            },
        };

        // If a rook is taken at its starting position, then set flag so player cant castle
        match removed_piece {
            None => {},
            Some(t) => {
                if t.piece == PieceType::ROOK && t.color == PieceColor::WHITE {
                    if to.x == 0 && to.y == 7 { self.left_white_rook_moved = true } else if to.x == 7 && to.y == 7 { self.right_white_rook_moved = true }
                }
                else if t.piece == PieceType::ROOK && t.color == PieceColor::BLACK {
                    if to.x == 0 && to.y == 0{ self.left_black_rook_moved = true } else if to.x == 7 && to.y == 0 { self.right_black_rook_moved = true }
                }
            },
        }
    }
}

impl Information {
    pub fn can_castle(&mut self, squares: &mut [Option<Piece>; 64], moves: &mut Vec<ChessMove>, kind: &Piece, wking_idx: &ChessMove, bking_idx: &ChessMove) {
        self.castle_moves.clear();
        
        // WHITE KING
        if kind.piece == PieceType::KING && kind.color == PieceColor::WHITE && !self.white_king_moved {
            let idx_one = ChessMove::new(1, 7).convert();
            let idx_two = ChessMove::new(2, 7).convert();
            let idx_three = ChessMove::new(3, 7).convert();

            if !self.left_white_rook_moved && squares[idx_one].is_none() && squares[idx_two].is_none() && squares[idx_three].is_none() {

                if moves.contains(&ChessMove::new(3, 7)) && !does_castle_check(squares, ChessMove::new(2, 7), ChessMove::new(4, 7), PieceColor::WHITE, wking_idx, bking_idx) {
                    moves.push(ChessMove::new(2, 7));
                    self.castle_moves.push(ChessMove::new(2, 7));
                }
            }

            let idx_one = ChessMove::new(5, 7).convert();
            let idx_two = ChessMove::new(6, 7).convert();

            if !self.right_white_rook_moved && squares[idx_one].is_none() && squares[idx_two].is_none() {

                if moves.contains(&ChessMove::new(5, 7)) && !does_castle_check(squares, ChessMove::new(6, 7), ChessMove::new(4, 7), PieceColor::WHITE, wking_idx, bking_idx) {
                    moves.push(ChessMove::new(6, 7));
                    self.castle_moves.push(ChessMove::new(6, 7));
                }
            }
        }

        // BLACK KING
        if kind.piece == PieceType::KING && kind.color == PieceColor::BLACK && !self.black_king_moved {
            let idx_one = ChessMove::new(1, 0).convert();
            let idx_two = ChessMove::new(2, 0).convert();
            let idx_three = ChessMove::new(3, 0).convert();

            if !self.left_black_rook_moved && squares[idx_one].is_none() && squares[idx_two].is_none() && squares[idx_three].is_none() {

                if moves.contains(&ChessMove::new(3, 0)) && !does_castle_check(squares, ChessMove::new(2, 0), ChessMove::new(4, 0), PieceColor::BLACK, wking_idx, bking_idx) {
                    moves.push(ChessMove::new(2, 0));
                    self.castle_moves.push(ChessMove::new(2, 0));
                }
            }

            let idx_one = ChessMove::new(5, 0).convert();
            let idx_two = ChessMove::new(6, 0).convert();

            if !self.right_black_rook_moved && squares[idx_one].is_none() && squares[idx_two].is_none() {

                if moves.contains(&ChessMove::new(5, 0)) && !does_castle_check(squares, ChessMove::new(6, 0), ChessMove::new(4, 0), PieceColor::BLACK, wking_idx, bking_idx) {
                    moves.push(ChessMove::new(6, 0));
                    self.castle_moves.push(ChessMove::new(6, 0));
                }
            }
        }
    }

    // If making a castle, moves the rook accordingly, based on which direction castling
    pub fn move_rook(&self, squares: &mut [Option<Piece>; 64], to: &ChessMove) {
        if self.castle_moves.contains(to) {
            let x1: i32;
            let y1 = to.y;

            let x2: i32;
            let y2 = to.y;
            if to.x == 2 {
                x1 = 3;
                x2 = 0;
            }
            else {
                x1 = 5;
                x2 = 7;
            }

            let rook_moved_to = ChessMove::new(x1, y1).convert();
            let rook_moved_from = ChessMove::new(x2, y2).convert();

            if y1 == 7 {
                squares[rook_moved_to] = Some(Piece::new(PieceType::ROOK, PieceColor::WHITE)); 
            }
            else {
                squares[rook_moved_to] = Some(Piece::new(PieceType::ROOK, PieceColor::BLACK));
            }

            squares[rook_moved_from] = None;
        }
    }
}

// It is not allowed to castle in to check, this functions simulates the castle move and sees if
// the player gets in check
fn does_castle_check(squares: &mut [Option<Piece>; 64], to: ChessMove, from: ChessMove, color: PieceColor, wking_idx: &ChessMove, bking_idx: &ChessMove) -> bool {
    let to_idx = to.convert();
    let from_idx = from.convert();
    let king_being_moved = squares[from_idx];
    let mut temp_wking_idx = *wking_idx;
    let mut temp_bking_idx = *bking_idx;

    if king_being_moved.unwrap().color == PieceColor::WHITE {
        temp_wking_idx = to;
    }
    else {
        temp_bking_idx = to;
    }

    squares[to_idx] = Some(Piece::new(PieceType::KING, color));
    squares[from_idx] = None;

    if check::is_check(&squares, color, &temp_wking_idx, &temp_bking_idx).0 == true {
        squares[from_idx] = Some(Piece::new(PieceType::KING, color));
        squares[to_idx] = None;
        return true;
    }

    squares[from_idx] = Some(Piece::new(PieceType::KING, color));
    squares[to_idx] = None;
    false
}
