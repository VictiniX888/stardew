use std::collections::VecDeque;

use macroquad::prelude::*;

use crate::{
    game::GameState,
    grid::Grid,
    item::{Item, ItemResource, ItemStack, WorldItem},
};

pub struct World {
    pub tile_objects: Grid<Option<TileObject>>,
    pub world_items: Vec<WorldItem>,
}

impl World {
    // Tile objects
    pub fn get_tile_object_pos_by_index(&self, index: TileObjectIndex) -> Vec2 {
        Vec2 {
            x: (index.0 % self.tile_objects.cols) as f32,
            y: (index.0 / self.tile_objects.cols) as f32,
        }
    }

    pub fn get_tile_object_from_index(&self, index: TileObjectIndex) -> &Option<TileObject> {
        self.tile_objects.get_by_index(index.0)
    }

    pub fn get_mut_tile_object_from_index(
        &mut self,
        index: TileObjectIndex,
    ) -> &mut Option<TileObject> {
        self.tile_objects.get_mut_by_index(index.0)
    }

    pub fn set_tile_object_with_index(&mut self, index: TileObjectIndex, tile_object: TileObject) {
        self.tile_objects.set_by_index(index.0, Some(tile_object));
    }

    pub fn remove_tile_object(&mut self, index: TileObjectIndex) -> Option<TileObject> {
        self.tile_objects.replace_by_index(index.0, None)
    }

    pub fn get_tile_object_index(&self, row: usize, col: usize) -> TileObjectIndex {
        TileObjectIndex(row * self.tile_objects.cols + col)
    }

    // World items
    pub fn spawn_world_items(&mut self, items: Vec<ItemStack>, pos: Vec2) {
        let items = items.into_iter().flat_map(|itemstack| {
            (0..itemstack.count).into_iter().map(move |_| WorldItem {
                item: itemstack.item.clone(),
                pos: Vec2 {
                    x: pos.x + rand::gen_range(-2.0, 2.0),
                    y: pos.y + rand::gen_range(-2.0, 2.0),
                },
            })
        });

        // Insert into sorted vec
        // Taken from https://doc.rust-lang.org/stable/std/primitive.slice.html#method.partition_point
        for item in items {
            let idx = self
                .world_items
                .partition_point(|other| other.pos.y <= item.pos.y);
            self.world_items.insert(idx, item);
        }
    }
}

#[derive(Clone, Copy)]
pub struct TileObjectIndex(usize);

#[derive(Clone)]
pub struct TileObject {
    hp: u32,
    pub kind: TileObjectKind,
}

impl TileObject {
    pub fn on_damage(&mut self, damage: u32, index: TileObjectIndex) -> Vec<WorldEvent> {
        match &mut self.kind {
            _ => {
                if self.hp == 0 {
                    vec![WorldEvent::TileObjectDestroy { tile_object: index }]
                } else {
                    vec![]
                }
            }
        }
    }

    pub fn on_destroy(&mut self, index: TileObjectIndex, pos: Vec2) -> Vec<WorldEvent> {
        let mut events = match &mut self.kind {
            TileObjectKind::Tree(TileObjectTree {
                kind: TileObjectTreeKind::TreeFullyGrown(tree),
                ..
            }) => tree.on_destroy(index),

            _ => vec![],
        };

        // Handle drops
        events.push(WorldEvent::ItemDrop {
            items: self.get_drops(),
            pos,
        });

        events
    }

    pub fn get_drops(&self) -> Vec<ItemStack> {
        match &self.kind {
            TileObjectKind::Tree(tree) => tree.get_drops(),
            _ => vec![],
        }
    }
}

#[derive(Clone)]
pub enum TileObjectKind {
    Tree(TileObjectTree),
}

#[derive(Clone)]
pub struct TileObjectTree {
    pub kind: TileObjectTreeKind,
}

impl TileObjectTree {
    pub fn get_drops(&self) -> Vec<ItemStack> {
        match &self.kind {
            TileObjectTreeKind::TreeFullyGrown(_) => vec![ItemStack {
                item: Item::Resource(ItemResource::Wood),
                count: 12,
            }],
            TileObjectTreeKind::TreeStump(_) => vec![ItemStack {
                item: Item::Resource(ItemResource::Wood),
                count: 5,
            }],
        }
    }
}

#[derive(Clone)]
pub enum TileObjectTreeKind {
    TreeFullyGrown(TreeFullyGrown),
    TreeStump(TreeStump),
}

#[derive(Clone)]
pub struct TreeFullyGrown;
impl TreeFullyGrown {
    pub fn new() -> TileObject {
        TileObject {
            hp: 10,
            kind: TileObjectKind::Tree(TileObjectTree {
                kind: TileObjectTreeKind::TreeFullyGrown(TreeFullyGrown),
            }),
        }
    }

    pub fn on_destroy(&self, index: TileObjectIndex) -> Vec<WorldEvent> {
        vec![WorldEvent::TileObjectReplace {
            tile_object: index,
            new_tile_object: TreeStump::new(),
        }]
    }
}

#[derive(Clone)]
pub struct TreeStump;
impl TreeStump {
    pub fn new() -> TileObject {
        TileObject {
            hp: 5,
            kind: TileObjectKind::Tree(TileObjectTree {
                kind: TileObjectTreeKind::TreeStump(TreeStump),
            }),
        }
    }
}

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
