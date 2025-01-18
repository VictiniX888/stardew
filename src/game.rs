use macroquad::prelude::*;

use crate::map::MapWorldData;

pub struct GameState {
    pub map_data: Option<MapWorldData>,

    pub player_pos: Vec2,
}

impl GameState {
    pub fn init() -> GameState {
        GameState {
            map_data: None,
            player_pos: vec2(0.0, 0.0),
        }
    }

    pub fn on_action(&mut self) {
        // TODO: check player equipped item
        // Currently assumes it is a hoe
        if self.map_data.is_none() {
            return;
        }

        let tiles = &mut self.map_data.as_mut().unwrap().tiles;
        let tile = tiles.get_mut(
            self.player_pos.y.round() as usize,
            self.player_pos.x.round() as usize,
        );

        tile.on_hoe();
    }
}
