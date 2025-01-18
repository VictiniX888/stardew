use macroquad::prelude::*;

use crate::{game::GameState, render::RenderState};

pub fn handle_input(game_state: &mut GameState, render_state: &mut RenderState) {
    // Player movement
    if is_key_down(KeyCode::Right) {
        game_state.player_pos.x += 1.0;
    }
    if is_key_down(KeyCode::Left) {
        game_state.player_pos.x -= 1.0;
    }
    if is_key_down(KeyCode::Down) {
        game_state.player_pos.y += 1.0;
    }
    if is_key_down(KeyCode::Up) {
        game_state.player_pos.y -= 1.0;
    }

    // Camera zoom
    if is_key_pressed(KeyCode::Period) {
        render_state.zoom += 1;
    }
    if is_key_pressed(KeyCode::Comma) {
        render_state.zoom -= 1;
        if render_state.zoom < 1 {
            render_state.zoom = 1;
        }
    }

    // Actions
    if is_mouse_button_pressed(MouseButton::Left) {
        game_state.on_action();
    }
}
