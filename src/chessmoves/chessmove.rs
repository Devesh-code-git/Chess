use crate::board::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ChessMove {
   pub x: i32,
   pub y: i32,
}

impl ChessMove {
    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    } 

    // This function converts the coordinate into a proper index for the board array
    pub fn convert(&self) -> usize {
        ((self.y * 8) + self.x) as usize
    }

    pub fn get_legal_moves(&self, kind: &Piece, board: &[Option<Piece>]) -> Vec<ChessMove> {
        match &kind.piece {
            PieceType::PAWN => return self.pawn_moves(kind.color, board),
            PieceType::ROOK => return self.rook_moves(kind.color, board),
            PieceType::KNIGHT => return self.knight_moves(kind.color, board),
            PieceType::BISHOP => return self.bishop_moves(kind.color, board),
            PieceType::QUEEN => return self.queen_moves(kind.color, board),
            PieceType::KING => return self.king_moves(kind.color, board),

        }
    }
}

// Another convert function that works with just points
fn convert_with_points(x: i32, y: i32) -> i32 {
    (y * 8) + x
}

// Pawn move validation checker and helpers
impl ChessMove {
    //--PAWN-//
    fn pawn_moves(&self, color: PieceColor, board: &[Option<Piece>]) -> Vec<ChessMove> { 
        if color == PieceColor::WHITE {
            return self.white_pawn(color, board);
        }
        else {
            return self.black_pawn(color, board);
        }
    }

    fn white_pawn(&self, color: PieceColor, board: &[Option<Piece>]) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let file = self.x;
        let rank = self.y;

        // Index of left diagonall from pawn
        let idx_one = ((rank - 1) * 8) + file - 1;
        // Index of right diagonall from pawn                              
        let idx_two = ((rank - 1) * 8) + file + 1;

        // If index one or two is in range, since a pawn can be at the edge of the board, and there
        // is a piece digaonally from the pawn, add that coordinate to the vector
        if (file - 1 >= 0) && (rank - 1 >= 0) {
            match &board[idx_one as usize] {
                None => {},
                Some(t) => {
                    if t.color != color { moves.push(ChessMove::new(file - 1, rank - 1)); }
                },
            };
        }

        if (file + 1 <= 7) && (rank - 1 >= 0) {
            match &board[idx_two as usize] {
                None => {},
                Some(t) => {
                    if t.color != color { moves.push(ChessMove::new(file + 1, rank - 1)); }
                },
            };
        }

        // If at the start, can move 2 places up
        if rank == 6 && board[(((rank - 1) * 8) + file) as usize].is_none() {
            let idx = ((rank - 2) * 8) + file;
            match &board[idx as usize] {
                None => moves.push(ChessMove::new(file, rank - 2)),
                Some(_t) => {},
            };
        }

        if rank != 0 {
            let idx = ((rank - 1) * 8) + file;
            match &board[idx as usize] {
                None => moves.push(ChessMove::new(file, rank - 1)),
                Some(_t) => {},
            };
        }       

        moves
    }

    fn black_pawn(&self, color: PieceColor, board: &[Option<Piece>]) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let file = self.x;
        let rank = self.y;

        // Index of left diagonall from pawn
        let idx_one = ((rank + 1) * 8) + file - 1;
        // Index of right diagonall from pawn                              
        let idx_two = ((rank + 1) * 8) + file + 1; 

        if (file - 1 >= 0) && (rank + 1 <= 7) {
            match &board[idx_one as usize] {
                None => {},
                Some(t) => {
                    if t.color != color { moves.push(ChessMove::new(file - 1, rank + 1)) }
                },
            };
        }

        if (file + 1 <= 7) && (rank + 1 <= 7) {
            match &board[idx_two as usize] {
                None => {},
                Some(t) => {
                    if t.color != color { moves.push(ChessMove::new(file + 1, rank + 1)) }
                },
            };
        }

        // If at the start, can move 2 places up
        if rank == 1 && board[(((rank + 1) * 8) + file) as usize].is_none() {
            let idx = ((rank + 2) * 8) + file;
            match &board[idx as usize] {
                None => moves.push(ChessMove::new(file, rank + 2)),
                Some(_t) => {},
            };
        }

        if rank != 7 {
            let idx = ((rank + 1) * 8) + file;
            match &board[idx as usize] {
                None => moves.push(ChessMove::new(file, rank + 1)),
                Some(_t) => {},
            };
        }

        moves
    }
}

// Move validation checkers
impl ChessMove { 
    //--ROOK--//
    fn rook_moves(&self, color: PieceColor, board: &[Option<Piece>]) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let file = self.x;
        let rank = self.y;

        // Each loop goes from the rooks current position straight outward from each direction until it hits
        // a piece, if no piece is hit during the loop, that means the rook can travel to that edge

        // Right
        for i in (file + 1)..8 {
            moves.push(ChessMove::new(i, rank));

            let idx = (i + (rank * 8)) as usize;
            match &board[idx] {
                None => {},
                Some(t) => {
                    if t.color == color { moves.pop(); }
                    break;
                },
            };
        } 

        // Left
        let mut i = file - 1;
        while i >= 0 {
            moves.push(ChessMove::new(i, rank));

            let idx = (i + (rank * 8)) as usize;
            match &board[idx] {
                None => {},
                Some(t) => {
                    if t.color == color { moves.pop(); }
                    break;
                },
            };

            i -= 1;
        }

        // Down
        for i in (rank + 1)..8 {
            moves.push(ChessMove::new(file, i));

            let idx = ((i * 8) + file) as usize;
            match &board[idx] {
                None => {},
                Some(t) => {
                    if t.color == color { moves.pop(); }
                    break;
                },
            };
        }

        // Up
        i = rank - 1;
        while i >= 0 {
            moves.push(ChessMove::new(file, i));

            let idx = ((i * 8) + file) as usize;
            match &board[idx] {
                None => {},
                Some(t) => {
                    if t.color == color { moves.pop(); }
                    break;
                },
            };

            i -= 1;
        }

        moves
    }

    //--KNIGHT--//
    fn knight_moves(&self, color: PieceColor, board: &[Option<Piece>]) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let file = self.x;
        let rank = self.y;
        let mut idx = convert_with_points(file + 1, rank - 2);

        // Generates 8 L-shaped directions of the knight, checks if indexes are valid and differnt
        // color piece is there, than pushes on to vector

        if (file + 1 <= 7) && (rank - 2 >= 0) {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file + 1, rank - 2)); } },
                None => moves.push(ChessMove::new(file + 1, rank - 2)),
            };
        }

        idx = convert_with_points(file + 2, rank - 1);
        if (file + 2 <= 7) && (rank - 1 >= 0) {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file + 2, rank - 1)); } },
                None => moves.push(ChessMove::new(file + 2, rank - 1)),
            };
        }

        idx = convert_with_points(file - 1, rank - 2);
        if (file - 1 >= 0) && (rank - 2 >= 0) {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file - 1, rank - 2)); } },
                None => moves.push(ChessMove::new(file - 1, rank - 2)),
            };
        }

        idx = convert_with_points(file - 2, rank - 1);
        if (file - 2 >= 0) && (rank - 1 >= 0) {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file - 2, rank - 1)); } },
                None => moves.push(ChessMove::new(file - 2, rank - 1)),
            };
        }

        idx = convert_with_points(file + 2, rank + 1);
        if (file + 2 <= 7) && (rank + 1 <= 7) {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file + 2, rank + 1)); } },
                None => moves.push(ChessMove::new(file + 2, rank + 1)),
            };
        }

        idx = convert_with_points(file + 1, rank + 2);
        if (file + 1 <= 7) && (rank + 2 <= 7) {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file + 1, rank + 2)); } },
                None => moves.push(ChessMove::new(file + 1, rank + 2)),
            };
        }

        idx = convert_with_points(file - 2, rank + 1);
        if (file - 2 >= 0) && (rank + 1 <= 7) {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file - 2, rank + 1)); } },
                None => moves.push(ChessMove::new(file - 2, rank + 1)),
            };
        }

        idx = convert_with_points(file - 1, rank + 2);
        if (file - 1 >= 0) && (rank + 2 <= 7) {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file - 1, rank + 2)); } },
                None => moves.push(ChessMove::new(file - 1, rank + 2)),
            };
        }
        
        moves
    }

    //--BISHOP--//
    fn bishop_moves(&self, color: PieceColor, board: &[Option<Piece>]) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let file = self.x;
        let rank = self.y;

        let mut vertical = rank - 1;

        //The looping logic is the same as the rooks, but each loop goes in one of the four digonall
        //directions, not straight

        //UP-RIGHT
        for i in (file + 1)..8 {
            if vertical < 0 {
                break;
            }

            moves.push(ChessMove::new(i, vertical));
            
            let idx = ((vertical * 8) + i) as usize;
            match &board[idx] {
                None => {},
                Some(t) => {
                    if t.color == color { moves.pop(); }
                    break;
                },
            };

            vertical -= 1;
        }

        vertical = rank - 1;
        let mut i = file - 1;

        //UP-LEFT
        while i >= 0 {
            if vertical < 0 {
                break;
            }

            moves.push(ChessMove::new(i, vertical));

            let idx = ((vertical * 8) + i) as usize; 
            match &board[idx] {
                None => {},
                Some(t) => {
                    if t.color == color { moves.pop(); }
                    break;
                },
            };

            i -= 1;
            vertical -= 1;
        }

        vertical = rank + 1;

        //Down-Right
        for i in (file + 1)..8 {
            if vertical > 7 {
                break;
            }

            moves.push(ChessMove::new(i, vertical));

            let idx = ((vertical * 8) + i) as usize;
            match &board[idx] {
                None => {},
                Some(t) => {
                    if t.color == color { moves.pop(); }
                    break;
                },
            };

            vertical += 1;
        }

        i = file - 1;
        vertical = rank + 1;

        //Down-Left
        while i >= 0 {
            if vertical > 7 {
                break;
            }

            moves.push(ChessMove::new(i, vertical));

            let idx = ((vertical * 8) + i) as usize; 
            match &board[idx] {
                None => {},
                Some(t) => {
                    if t.color == color { moves.pop(); }
                    break;
                },
            };

            i -= 1;
            vertical += 1;
        }

        moves
    }

    //--QUEEN--//
    fn queen_moves(&self, color: PieceColor, board: &[Option<Piece>]) -> Vec<ChessMove> {
        // Since queen combines both rook and bishop movements, generate both sets of moves, and
        // then combine them
        let mut rook = self.rook_moves(color, board);
        let mut bishop = self.bishop_moves(color, board);

        rook.append(&mut bishop);

        rook
    }

    //--KING--//
    fn king_moves(&self, color: PieceColor, board: &[Option<Piece>]) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let file = self.x;
        let rank = self.y;
        let mut idx = convert_with_points(file, rank - 1);

        // Generates movement options of the king, sees if it fits the index and a opposite piece is
        // there

        if rank - 1 >= 0 {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file, rank - 1)); } },
                None => moves.push(ChessMove::new(file, rank - 1)),
            };
        }

        idx = convert_with_points(file + 1, rank - 1);
        if (file + 1 <= 7) && (rank - 1 >= 0) {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file + 1, rank - 1)); } },
                None => moves.push(ChessMove::new(file + 1, rank - 1)),
            };
        }

        idx = convert_with_points(file - 1, rank - 1);
        if (file - 1 >= 0) && (rank - 1 >= 0) {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file - 1, rank - 1)); } },
                None => moves.push(ChessMove::new(file - 1, rank - 1)),
            };
        }

        idx = convert_with_points(file + 1, rank);
        if file + 1 <= 7 {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file + 1, rank)); } },
                None => moves.push(ChessMove::new(file + 1, rank)),
            };
        }

        idx = convert_with_points(file - 1, rank);
        if file - 1 >= 0 {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file - 1, rank)); } },
                None => moves.push(ChessMove::new(file - 1, rank)),
            };
        }

        idx = convert_with_points(file, rank + 1);
        if rank + 1 <= 7 {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file, rank + 1)); } },
                None => moves.push(ChessMove::new(file, rank + 1)),
            };
        }

        idx = convert_with_points(file + 1, rank + 1);
        if (file + 1 <= 7) && (rank + 1 <= 7) {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file + 1, rank + 1)); } },
                None => moves.push(ChessMove::new(file + 1, rank + 1)),
            };
        }

        idx = convert_with_points(file - 1, rank + 1);
        if (file - 1 >= 0) && (rank + 1 <= 7) {
            match &board[idx as usize] {
                Some(t) => { if t.color != color { moves.push(ChessMove::new(file - 1, rank + 1)); } },
                None => moves.push(ChessMove::new(file - 1, rank + 1)),
            };
        }

        moves
    }
}
