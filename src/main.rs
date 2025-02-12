use event::process_events;
use macroquad::prelude::*;

use game::GameState;
use input::handle_input;
use render::RenderState;
use system::pickup_world_items;

mod event;
mod game;
mod grid;
mod input;
mod item;
mod map;
mod player;
mod render;
mod system;
mod tile_object;
mod world;

#[macroquad::main("Stardew Valley")]
async fn main() {
    // Seed random
    rand::srand(macroquad::miniquad::date::now() as _);

    // Init
    let mut game_state = GameState::init();
    let mut renderer = RenderState::init();
    map::load_map(map::MapType::Farm, &mut game_state, &mut renderer).await;
    renderer.load_player_texture().await;
    renderer.load_tile_object_textures().await;
    renderer.load_item_textures().await;
    build_textures_atlas();

    loop {
        // Input
        handle_input(&mut game_state, &mut renderer);

        // Systems
        pickup_world_items(&mut game_state);

        // Process events
        process_events(&mut game_state);

        clear_background(BLACK);
        // Draw map
        renderer.render_world(&game_state);
        renderer.render_hotbar(&game_state);

        // Show FPS
        draw_text(&get_fps().to_string(), 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
