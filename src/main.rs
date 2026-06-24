mod board;
mod chessmoves;
mod rendering;
mod assets;
use crate::board::*;
use crate::assets::Assets;
use macroquad::prelude::*;

#[macroquad::main("Devesh's Chess")]
async fn main() {
    let mut game = State::new();
    let pieces_images = Assets::new().await;

    let mut x1: i32 = 0;
    let mut y1: i32 = 0;
    let mut x2: i32 = 0;
    let mut y2: i32 = 0;

    let mut moved_x1 = 0;
    let mut moved_y1 = 0;
    let mut moved_x2 = 0;
    let mut moved_y2 = 0;

    let mut pressed_once = false;
    let mut highlight = false;
    let mut moved = false;
    let mut game_ended = false;
    let mut selection_screen = false;

    // Main game loop
    loop { 
        rendering::draw_tiles();

        if moved {
            rendering::moved_highlight(moved_x1 as f32, moved_y1 as f32, moved_x2 as f32, moved_y2 as f32);
        }

        if highlight && game_ended == false && selection_screen == false {
            rendering::highlight_square(x1, y1, game.moves(), game.squares());
        }

        if game.checkmate_stalemate().1 == true {
            match game.stalemate_idx() {
                None => {},
                Some(coordinate) => {
                    let x = coordinate.x as f32;
                    let y = coordinate.y as f32;
                    rendering::stalemate_highlight(x, y);
                    game_ended = true;
                }
            }
        }
        else if game.in_check().0 == true {
            let x = game.in_check().1;
            let y = game.in_check().2;

            if game.checkmate_stalemate().0 == true {
                rendering::checkmate_highlight(x, y);
                game_ended = true;
            }
            else {
                rendering::check_highlight(x, y);
            }
        } 

        rendering::draw_pieces(game.squares(), &pieces_images);

        if is_mouse_button_pressed(MouseButton::Left) && game_ended == false && selection_screen == false {
            if rendering::input_calculations(&mut pressed_once, &mut highlight, &mut moved, (&mut x1, &mut y1), (&mut x2, &mut y2), &mut game) {
                moved_x1 = x1;
                moved_y1 = y1;
                moved_x2 = x2;
                moved_y2 = y2;
            }
        }

        if game.pawn_promotion_idx().is_some() {
            let idx = game.pawn_promotion_idx().unwrap();
            let mut color: PieceColor = PieceColor::WHITE;

            if idx.y == 7 {
                color = PieceColor::BLACK;
            }

            let flag = rendering::selection_screen(color, &pieces_images, &mut game);

            if flag == false {
                selection_screen = true;
            }
            else {
                selection_screen = false;
            }
        }

        next_frame().await
    }
}
