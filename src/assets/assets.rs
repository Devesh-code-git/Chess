use macroquad::prelude::*;

pub struct Assets {
    pub white_pawn: Texture2D,
    pub white_knight: Texture2D,
    pub white_rook: Texture2D,
    pub white_bishop: Texture2D,
    pub white_queen: Texture2D,
    pub white_king: Texture2D,

    pub black_pawn: Texture2D,
    pub black_knight: Texture2D,
    pub black_rook: Texture2D,
    pub black_bishop: Texture2D,
    pub black_queen: Texture2D,
    pub black_king: Texture2D,
}

impl Assets {
    pub async fn new() -> Self {
        Self {
            white_pawn: load_texture("src/assets/WHITE_PAWN.png").await.unwrap(),
            white_knight: load_texture("src/assets/WHITE_KNIGHT.png").await.unwrap(),
            white_rook: load_texture("src/assets/WHITE_ROOK.png").await.unwrap(),
            white_bishop: load_texture("src/assets/WHITE_BISHOP.png").await.unwrap(),
            white_queen: load_texture("src/assets/WHITE_QUEEN.png").await.unwrap(),
            white_king: load_texture("src/assets/WHITE_KING.png").await.unwrap(),

            black_pawn: load_texture("src/assets/BLACK_PAWN.png").await.unwrap(),
            black_knight: load_texture("src/assets/BLACK_KNIGHT.png").await.unwrap(),
            black_rook: load_texture("src/assets/BLACK_ROOK.png").await.unwrap(),
            black_bishop: load_texture("src/assets/BLACK_BISHOP.png").await.unwrap(),
            black_queen: load_texture("src/assets/BLACK_QUEEN.png").await.unwrap(),
            black_king: load_texture("src/assets/BLACK_KING.png").await.unwrap(),
        }
    }
}
