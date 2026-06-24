#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceColor {
    BLACK,
    WHITE,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub color: PieceColor,
    pub piece: PieceType,
}

impl Piece {
    pub fn new(piece: PieceType, color: PieceColor) -> Self {
        Self {color, piece}
    }
}
