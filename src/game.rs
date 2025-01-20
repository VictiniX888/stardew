use macroquad::prelude::*;

use crate::map::{MapWorldData, TileObject};

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

        let x = self.player_pos.x.round() as usize;
        let y = self.player_pos.y.round() as usize;

        let tile_objects = &mut self.map_data.as_mut().unwrap().objects;
        let tile_object = tile_objects.get_mut(y, x);

        tile_object.to_owned().on_axe(self);

        let tiles = &mut self.map_data.as_mut().unwrap().tiles;
        let tile = tiles.get_mut(y, x);

        // tile.on_hoe();
    }

    pub fn on_axe(&mut self, tile_object: &mut TileObject) {}
}
