use std::collections::VecDeque;

use macroquad::math::Vec2;

use crate::{
    game::GameState,
    item::{Item, ItemStack},
    tile_object::{TileObject, TileObjectIndex},
    world::World,
};

pub enum WorldEvent {
    // Item
    ItemUse {
        item: Item,
        target: TileObjectIndex,
    },

    // Tile object
    TileObjectDamage {
        tile_object: TileObjectIndex,
        damage: u32,
    },
    TileObjectDestroy {
        tile_object: TileObjectIndex,
    },
    TileObjectReplace {
        tile_object: TileObjectIndex,
        new_tile_object: TileObject,
    },

    // World items
    ItemDrop {
        items: Vec<ItemStack>,
        pos: Vec2,
    },
}

impl WorldEvent {
    pub fn process(self, world: &mut World) -> Vec<WorldEvent> {
        match self {
            WorldEvent::ItemUse { item, target } => item.on_use(target, world),

            WorldEvent::TileObjectDamage {
                tile_object,
                damage,
            } => {
                let Some(object) = world.get_mut_tile_object_from_index(tile_object) else {
                    return vec![];
                };

                object.hp = object.hp.saturating_sub(damage);

                object.on_damage(damage, tile_object)
            }

            WorldEvent::TileObjectDestroy { tile_object } => {
                let Some(mut old) = world.remove_tile_object(tile_object) else {
                    return vec![];
                };

                let pos = world.get_tile_object_pos_by_index(tile_object);
                old.on_destroy(tile_object, pos)
            }

            WorldEvent::TileObjectReplace {
                tile_object,
                new_tile_object,
            } => {
                world.set_tile_object_with_index(tile_object, new_tile_object);
                vec![]
            }

            WorldEvent::ItemDrop { items, pos } => {
                world.spawn_world_items(items, pos);
                vec![]
            }
        }
    }
}

pub trait EventQueue {
    fn process(&mut self, world: &mut World);
}

impl EventQueue for VecDeque<WorldEvent> {
    fn process(&mut self, world: &mut World) {
        while let Some(event) = self.pop_front() {
            self.extend(event.process(world));
        }
    }
}

pub fn process_events(game_state: &mut GameState) {
    game_state
        .event_queue
        .process(game_state.world.as_mut().unwrap());
}
