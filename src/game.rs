use std::collections::VecDeque;

use macroquad::prelude::*;

use crate::{
    event::WorldEvent,
    item::{Axe, Item, ItemTool},
    map::MapWorldData,
    world::World,
};

pub struct GameState {
    pub map_data: Option<MapWorldData>,

    pub player_pos: Vec2,

    pub world: Option<World>,
    pub event_queue: VecDeque<WorldEvent>,
}

impl GameState {
    pub fn init() -> GameState {
        GameState {
            map_data: None,
            player_pos: vec2(0.0, 0.0),
            world: None,
            event_queue: VecDeque::new(),
        }
    }

    pub fn on_action(&mut self) {
        // TODO: check player equipped item
        // Currently assumes it is a hoe
        if self.world.is_none() || self.map_data.is_none() {
            return;
        }

        let x = self.player_pos.x.round() as usize;
        let y = self.player_pos.y.round() as usize;

        self.event_queue.push_back(WorldEvent::ItemUse {
            item: Item::Tool(ItemTool::Axe(Axe {})),
            target: self.world.as_ref().unwrap().get_tile_object_index(y, x),
        });
    }
}
