use game::GameState;
use input::handle_input;
use macroquad::prelude::*;
use render::RenderState;

mod game;
mod grid;
mod input;
mod map;
mod render;

#[macroquad::main("Stardew Valley")]
async fn main() {
    let mut game_state = GameState::init();
    let mut renderer = RenderState::init();
    renderer.load_player_texture().await;
    map::load_map(map::MapType::Farm, &mut game_state, &mut renderer).await;
    build_textures_atlas();

    loop {
        // Input
        handle_input(&mut game_state, &mut renderer);

        clear_background(RED);

        // Draw map
        renderer.render_world(&game_state);

        // Show FPS
        draw_text(&get_fps().to_string(), 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
