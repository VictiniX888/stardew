use macroquad::math::Vec2;

use crate::{
    event::WorldEvent,
    item::{Item, ItemResource, ItemStack},
};

#[derive(Clone, Copy)]
pub struct TileObjectIndex(pub usize);

#[derive(Clone)]
pub struct TileObject {
    pub hp: u32,
    pub kind: TileObjectKind,
}

impl TileObject {
    pub fn on_damage(&mut self, _damage: u32, index: TileObjectIndex) -> Vec<WorldEvent> {
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
