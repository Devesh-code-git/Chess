use macroquad::prelude::*;
use crate::board::*;
use crate::assets::Assets;
use crate::chessmoves::*;

const LIGHT_TILE: Color = Color::from_rgba(106, 128, 145, 255);
const DARK_TILE: Color = Color::from_rgba(225, 230, 235, 255);
const SELECT_HIGHLIGHT: Color = Color::from_rgba(245, 200, 95, 200);
const LEGAL_HIGHLIGHT: Color = Color::from_rgba(110, 190, 120, 185);
const CAPTURE_HIGHLIGHT: Color = Color::from_rgba(220, 100, 100, 200);
const MOVED_HIGHLIGHT: Color = Color::from_rgba(120, 180, 240, 200);
const CHECK_HIGHLIGHT: Color = Color::from_rgba(235, 85, 65, 200);
const CHECKMATE_HIGHLIGHT: Color = Color::from_rgba(255, 0, 0, 255);
const STALEMATE_HIGHLIGHT: Color = Color::from_rgba(135, 135, 135, 255);

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.0;
const SQUARE_SIZE: f32 = 100.0;

// Draws the piece selection screen onto the board for pawn promotions
pub fn selection_screen(color: PieceColor, assets: &Assets, game: &mut State) -> bool {
    draw_rectangle(WIDTH / 2.0 - 280.0, HEIGHT / 2.0 - 105.0, 560.0, 210.0, BLACK);
    draw_rectangle(WIDTH / 2.0 - 275.0, HEIGHT / 2.0 - 100.0, 550.0, 200.0, DARK_TILE);

    let box1_x = WIDTH / 2.0 - 215.0;
    let box2_x = WIDTH / 2.0 - 105.0;
    let box3_x = WIDTH / 2.0 + 5.0;
    let box4_x = WIDTH / 2.0 + 115.0;
    let box_y = HEIGHT / 2.0 - 50.0;

    // Draws the boxes that holds the pieces
    draw_rectangle(box1_x, box_y, SQUARE_SIZE, SQUARE_SIZE, LIGHT_TILE);
    draw_rectangle(box2_x, box_y, SQUARE_SIZE, SQUARE_SIZE, LIGHT_TILE);
    draw_rectangle(box3_x, box_y, SQUARE_SIZE, SQUARE_SIZE, LIGHT_TILE);
    draw_rectangle(box4_x, box_y, SQUARE_SIZE, SQUARE_SIZE, LIGHT_TILE);

    let mouse_x = mouse_position().0;
    let mouse_y = mouse_position().1;

    // Highlights box if mouse is over it, if pressed inside updates board based on decision
    if mouse_x >= box1_x && mouse_x <= box1_x + 100.0 && mouse_y >= box_y && mouse_y <= box_y + 100.0 {
        draw_rectangle(box1_x, box_y, SQUARE_SIZE, SQUARE_SIZE, SELECT_HIGHLIGHT);

        if is_mouse_button_pressed(MouseButton::Left) {
            game.set_pawn_promotion(Piece::new(PieceType::QUEEN, color));
            return true;
        }
    }
    else if mouse_x >= box2_x && mouse_x <= box2_x + 100.0 && mouse_y >= box_y && mouse_y <= box_y + 100.0 {
        draw_rectangle(box2_x, box_y, SQUARE_SIZE, SQUARE_SIZE, SELECT_HIGHLIGHT);

        if is_mouse_button_pressed(MouseButton::Left) {
            game.set_pawn_promotion(Piece::new(PieceType::BISHOP, color));
            return true;
        }
    }
    else if mouse_x >= box3_x && mouse_x <= box3_x + 100.0 && mouse_y >= box_y && mouse_y <= box_y + 100.0 {
        draw_rectangle(box3_x, box_y, SQUARE_SIZE, SQUARE_SIZE, SELECT_HIGHLIGHT);

        if is_mouse_button_pressed(MouseButton::Left) {
            game.set_pawn_promotion(Piece::new(PieceType::ROOK, color));
            return true;
        }
    }
    else if mouse_x >= box4_x && mouse_x <= box4_x + 100.0 && mouse_y >= box_y && mouse_y <= box_y + 100.0 {
        draw_rectangle(box4_x, box_y, SQUARE_SIZE, SQUARE_SIZE, SELECT_HIGHLIGHT);

        if is_mouse_button_pressed(MouseButton::Left) {
            game.set_pawn_promotion(Piece::new(PieceType::KNIGHT, color));
            return true;
        }
    }

    // Draws the pieces
    if color == PieceColor::WHITE {
        draw_texture_ex(&assets.white_queen, WIDTH / 2.0 - 215.0, HEIGHT / 2.0 - 50.0, WHITE, piece_dimensions());
        draw_texture_ex(&assets.white_bishop, WIDTH / 2.0 - 105.0, HEIGHT / 2.0 - 50.0, WHITE, piece_dimensions());
        draw_texture_ex(&assets.white_rook, WIDTH / 2.0 + 5.0, HEIGHT / 2.0 - 50.0, WHITE, piece_dimensions());
        draw_texture_ex(&assets.white_knight, WIDTH / 2.0 + 115.0, HEIGHT / 2.0 - 50.0, WHITE, piece_dimensions());
    }
    else {
        draw_texture_ex(&assets.black_queen, WIDTH / 2.0 - 215.0, HEIGHT / 2.0 - 50.0, WHITE, piece_dimensions());
        draw_texture_ex(&assets.black_bishop, WIDTH / 2.0 - 105.0, HEIGHT / 2.0 - 50.0, WHITE, piece_dimensions());
        draw_texture_ex(&assets.black_rook, WIDTH / 2.0 + 5.0, HEIGHT / 2.0 - 50.0, WHITE, piece_dimensions());
        draw_texture_ex(&assets.black_knight, WIDTH / 2.0 + 115.0, HEIGHT / 2.0 - 50.0, WHITE, piece_dimensions());
    }

    false
}

// Function to draw the tiles to the screen
pub fn draw_tiles() {
    let mut tile = false;

    request_new_screen_size(WIDTH, HEIGHT);
    clear_background(LIGHT_TILE);

    for i in 0..8 {
        let y = SQUARE_SIZE * (i as f32);

        for j in 0..8 {
            let x = SQUARE_SIZE * (j as f32);

            if tile == false && (j + 1) % 2 != 0 {
                draw_rectangle(x, y, SQUARE_SIZE, SQUARE_SIZE, DARK_TILE);
            }
            else if tile && (j + 1) % 2 == 0 {
                draw_rectangle(x, y, SQUARE_SIZE, SQUARE_SIZE, DARK_TILE);
            }

            if j == 7 {
                tile = !tile;
            }
        }
    }
}



// Function to draw the pieces to the screen
pub fn draw_pieces(board: &[Option<Piece>], assets: &Assets) {
    for i in 0..64 {
        let rank = (i / 8) as f32;
        let file = (i % 8) as f32;

        match &board[i] {
            None => {}
            Some(t) => {
                match t.color {
                    PieceColor::WHITE => draw_white(t, rank, file, assets),
                    PieceColor::BLACK => draw_black(t, rank, file, assets),
                }
            }
        };
    }
}

fn draw_white(kind: &Piece, file: f32, rank: f32, assets: &Assets) {
    match kind.piece {
        PieceType::PAWN => draw_texture_ex(&assets.white_pawn, rank * SQUARE_SIZE, file * SQUARE_SIZE, WHITE, piece_dimensions()),
        PieceType::KNIGHT => draw_texture_ex(&assets.white_knight, rank * SQUARE_SIZE, file * SQUARE_SIZE, WHITE, piece_dimensions()),
        PieceType::ROOK => draw_texture_ex(&assets.white_rook, rank * SQUARE_SIZE, file * SQUARE_SIZE, WHITE, piece_dimensions()),
        PieceType::BISHOP => draw_texture_ex(&assets.white_bishop, rank * SQUARE_SIZE, file * SQUARE_SIZE, WHITE, piece_dimensions()),
        PieceType::QUEEN => draw_texture_ex(&assets.white_queen, rank * SQUARE_SIZE, file * SQUARE_SIZE, WHITE, piece_dimensions()),
        PieceType::KING => draw_texture_ex(&assets.white_king, rank * SQUARE_SIZE, file * SQUARE_SIZE, WHITE, piece_dimensions()),
    };
}

fn draw_black(kind: &Piece, file: f32, rank: f32, assets: &Assets) {
    match kind.piece {
        PieceType::PAWN => draw_texture_ex(&assets.black_pawn, rank * SQUARE_SIZE, file * SQUARE_SIZE, WHITE, piece_dimensions()),
        PieceType::KNIGHT => draw_texture_ex(&assets.black_knight, rank * SQUARE_SIZE, file * SQUARE_SIZE, WHITE, piece_dimensions()),
        PieceType::ROOK => draw_texture_ex(&assets.black_rook, rank * SQUARE_SIZE, file * SQUARE_SIZE, WHITE, piece_dimensions()),
        PieceType::BISHOP => draw_texture_ex(&assets.black_bishop, rank * SQUARE_SIZE, file * SQUARE_SIZE, WHITE, piece_dimensions()),
        PieceType::QUEEN => draw_texture_ex(&assets.black_queen, rank * SQUARE_SIZE, file * SQUARE_SIZE, WHITE, piece_dimensions()),
        PieceType::KING => draw_texture_ex(&assets.black_king, rank * SQUARE_SIZE, file * SQUARE_SIZE, WHITE, piece_dimensions()),
    };
}

// Function that returns a DrawTextureParams struct with the dimensions for a piece
fn piece_dimensions() -> DrawTextureParams {
    DrawTextureParams {
        dest_size: Some(vec2(100.0, 100.0)),
        ..Default::default()
    }
}

pub fn highlight_square(start_x: i32, start_y: i32, moves: &Vec<ChessMove>, board: &[Option<Piece>]) {
    draw_rectangle((start_x as f32) * SQUARE_SIZE, (start_y as f32) * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE, SELECT_HIGHLIGHT);

    for mv in moves {
        let x = mv.get_x() as f32;
        let y = mv.get_y() as f32;
        let idx = mv.convert();

        match &board[idx] {
            None => draw_rectangle(x * SQUARE_SIZE, y * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE, LEGAL_HIGHLIGHT),
            Some(_t) => draw_rectangle(x * SQUARE_SIZE, y * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE, CAPTURE_HIGHLIGHT),
        }
    }
}

pub fn moved_highlight(x1: f32, y1: f32, x2: f32, y2: f32) {
    draw_rectangle(x1 * SQUARE_SIZE, y1 * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE, MOVED_HIGHLIGHT);
    draw_rectangle(x2 * SQUARE_SIZE, y2 * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE, MOVED_HIGHLIGHT);
}

pub fn check_highlight(x: f32, y: f32) {
    draw_rectangle(x * SQUARE_SIZE, y * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE, CHECK_HIGHLIGHT);
}

pub fn checkmate_highlight(x: f32, y: f32) {
    draw_rectangle(x * SQUARE_SIZE, y * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE, CHECKMATE_HIGHLIGHT);
}

pub fn stalemate_highlight(x: f32, y: f32) {
    draw_rectangle(x * SQUARE_SIZE, y * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE, STALEMATE_HIGHLIGHT);
}

// Function that checks if it is valid to continues with the given inputs, also checks if its valid
// to put the highlight of the square
pub fn input_calculations(pressed_once: &mut bool, highlight: &mut bool, moved: &mut bool, start: (&mut i32, &mut i32), end: (&mut i32, &mut i32), game: &mut State) -> bool {
    if *pressed_once {
        *end.0 = (mouse_position().0 / 100.0) as i32;
        *end.1 = (mouse_position().1 / 100.0) as i32;

        let idx = ((*end.1 * 8) + *end.0) as usize;

        match game.piece_at(idx) {
            None => {
                *pressed_once = false;
                *highlight = false;
                if game.make_move(ChessMove::new(*end.0, *end.1), ChessMove::new(*start.0, *start.1)) {
                    *moved = true;
                    return true;
                }
            },

            Some(&t) => {
                if &t.color == game.turn() {
                    *start.0 = *end.0;
                    *start.1 = *end.1;
                    game.get_legal_moves(ChessMove::new(*start.0, *start.1));
                }
                else {
                    *pressed_once = false;
                    *highlight = false;
                    if game.make_move(ChessMove::new(*end.0, *end.1), ChessMove::new(*start.0, *start.1)) {
                        *moved = true;
                        return true;
                    }
                }
            },
        };
    }
    else if *pressed_once == false {
        *start.0 = (mouse_position().0 / 100.0) as i32;
        *start.1 = (mouse_position().1 / 100.0) as i32;

        let idx = ((*start.1 * 8) + *start.0) as usize;

        match game.piece_at(idx) {
            None => {},
            Some(t) => {
                if &t.color == game.turn() {
                    *pressed_once = true;
                    *highlight = true;
                    game.get_legal_moves(ChessMove::new(*start.0, *start.1));
                }
            },
        };
    }

    false
}
