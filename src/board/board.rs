use crate::chessmoves::*;
use crate::board::*;

pub struct State {
    squares: [Option<Piece>; 64],
    turn: PieceColor,
    moves: Vec<ChessMove>, // Holds the legal moves for the current picked piece
    white_king_idx: ChessMove,
    black_king_idx: ChessMove,
    in_check: (bool, f32, f32),
    checkmate_stalemate: (bool, bool),
    stalemate_idx: Option<ChessMove>,
    half_move_count: u32,

    castling_information: castle::Information,
    en_passant_idx: Option<ChessMove>, // Holds the coordinate of the square with the pawn
    pawn_promotion_idx: Option<ChessMove>,// Holds the coordinate of the pawn thats being promoted
}

impl State {
    // Constructor
    pub fn new() -> Self {
        let starting_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"; //Using the firt part
                                                                               //of a FEN to show
                                                                               //starting position

        let mut board: [Option<Piece>; 64] = [None; 64];
        let mut idx = 0;
        let mut dont_update = false;

        for i in starting_position.chars() {
            match i {
                'p' => board[idx] = Some(Piece{color: PieceColor::BLACK, piece: PieceType::PAWN}),
                'r' => board[idx] = Some(Piece{color: PieceColor::BLACK, piece: PieceType::ROOK}),
                'n' => board[idx] = Some(Piece{color: PieceColor::BLACK, piece: PieceType::KNIGHT}),
                'b' => board[idx] = Some(Piece{color: PieceColor::BLACK, piece: PieceType::BISHOP}),
                'q' => board[idx] = Some(Piece{color: PieceColor::BLACK, piece: PieceType::QUEEN}),
                'k' => board[idx] = Some(Piece{color: PieceColor::BLACK, piece: PieceType::KING}),

                'P' => board[idx] = Some(Piece{color: PieceColor::WHITE, piece: PieceType::PAWN}),
                'R' => board[idx] = Some(Piece{color: PieceColor::WHITE, piece: PieceType::ROOK}),
                'N' => board[idx] = Some(Piece{color: PieceColor::WHITE, piece: PieceType::KNIGHT}),
                'B' => board[idx] = Some(Piece{color: PieceColor::WHITE, piece: PieceType::BISHOP}),
                'Q' => board[idx] = Some(Piece{color: PieceColor::WHITE, piece: PieceType::QUEEN}),
                'K' => board[idx] = Some(Piece{color: PieceColor::WHITE, piece: PieceType::KING}),

                '1'..='8' => {
                    let n = i.to_digit(10).unwrap();

                    for _j in 0..n {
                        board[idx] = None;
                        idx += 1;
                    }

                    dont_update = true;
                },

                '/' => dont_update = true,
                _ => dont_update = true,

            }

            if dont_update {
                dont_update = false;
            }
            else {
                idx += 1;
            }
        }

        Self {
            squares: board, 
            turn: PieceColor::WHITE, 
            moves: Vec::new(),
            white_king_idx: ChessMove::new(4, 7),
            black_king_idx: ChessMove::new(4, 0),
            in_check: (false, 0.0, 0.0),
            checkmate_stalemate: (false, false),
            stalemate_idx: None,
            half_move_count: 0,
            castling_information: castle::Information::new(),
            en_passant_idx: None,
            pawn_promotion_idx: None,
        }
    }
}

// Getters
impl State {
    pub fn squares(&self) -> &[Option<Piece>]{
        &self.squares
    }

    pub fn turn(&self) -> &PieceColor {
        &self.turn
    }

    pub fn moves(&self) -> &Vec<ChessMove> {
        &self.moves
    }

    pub fn pawn_promotion_idx(&self) -> &Option<ChessMove> {
        &self.pawn_promotion_idx
    }

    pub fn checkmate_stalemate(&self) -> &(bool, bool) {
        &self.checkmate_stalemate
    }

    pub fn in_check(&self) -> &(bool, f32, f32) {
        &self.in_check
    }

    pub fn stalemate_idx(&self) -> Option<ChessMove> {
        self.stalemate_idx
    }
}

impl State {
    pub fn set_pawn_promotion(&mut self, kind: Piece) {
       let idx = self.pawn_promotion_idx.unwrap().convert();
       self.squares[idx] = Some(kind);
       self.pawn_promotion_idx = None;
       self.in_check = check::is_check(&self.squares, self.turn, &self.white_king_idx, &self.black_king_idx);
    }
}

impl State {
    pub fn get_legal_moves(&mut self, mv: ChessMove) {
        self.moves.clear();

        let idx = mv.convert();
        let kind = match self.piece_at(idx) {
            None => return,
            Some(t) => *t,
        };

        let mut moves = mv.get_legal_moves(&kind, &self.squares);

        // Checking if can perfrom en_passant and adding it to the move list
        if kind.piece == PieceType::PAWN {
            match self.en_passant_idx {
                None => {},
                Some(mv) => {
                    if kind.color == PieceColor::WHITE {
                        moves.push(ChessMove::new(mv.x, mv.y - 1));
                    }
                    else {
                        moves.push(ChessMove::new(mv.x, mv.y + 1));
                    }
                },
            };
        }

        moves = check::filter_moves(&mut self.squares, &moves, mv, &self.en_passant_idx, &self.turn, &self.white_king_idx, &self.black_king_idx); 

        if !self.in_check.0 {
            self.castling_information.can_castle(&mut self.squares, &mut moves, &kind, &self.white_king_idx, &self.black_king_idx);
        }

        self.moves = moves;
    }

    pub fn make_move(&mut self, to: ChessMove, from: ChessMove) -> bool {
        // If picked move is not in legal moves list, false automatically
        if self.moves.contains(&to) == false {
            return false;
        }

        let kind = match self.piece_at(from.convert()) {
            None => return false,
            Some(t) => *t,
        };

        let removed_piece = self.squares[to.convert()]; // Could either be some or none

        // According to rules if a pawn is moved or piece is taken, than we reset the half move
        // count
        if kind.piece == PieceType::PAWN || removed_piece.is_some() {
            self.half_move_count = 0;
        }
        else {
            self.half_move_count += 1;
        }

        // Moving the piece
        self.squares[to.convert()] = Some(kind);
        self.squares[from.convert()] = None;
 
        let kind = match self.piece_at(to.convert()) {
            None => return false,
            Some(t) => *t,
        };

        // These functions are related to castling, and updating king positions
        self.castling_information.set_flags(&kind, removed_piece, &to, &from, &mut self.white_king_idx, &mut self.black_king_idx);
        self.castling_information.move_rook(&mut self.squares, &to);

        let file = to.x;
        let rank = to.y;
        let kind = *self.piece_at(to.convert()).unwrap();

        pawn::do_en_passant(&mut self.squares, &file, &rank, &kind, &self.en_passant_idx);
        pawn::pawn_promotion(&mut self.pawn_promotion_idx, &file, &rank, &kind);
       
        // Changeing turns
        self.change_turns();

        self.in_check = check::is_check(&mut self.squares, self.turn, &self.white_king_idx, &self.black_king_idx);
        self.checkmate_stalemate = check::is_checkmate_or_stalemate(&mut self.squares, self.turn, &mut self.stalemate_idx, self.in_check.0, &self.en_passant_idx, 
                                                                    &self.white_king_idx, &self.black_king_idx);
 
        if kind.piece == PieceType::PAWN && pawn::en_passant(&self.squares, &to, &from, &self.turn) {
            self.en_passant_idx = Some(to);
        }
        else {
            self.en_passant_idx = None;
        }

        true
    }
}

impl State {
    fn change_turns(&mut self) {
        match self.turn {
            PieceColor::WHITE => self.turn = PieceColor::BLACK,
            PieceColor::BLACK => self.turn = PieceColor::WHITE,
        }
    }

    pub fn piece_at(&self, idx: usize) -> Option<&Piece> {
        self.squares[idx].as_ref()
    }
}
