use macroquad::math::Vec2;

use crate::{
    event::WorldEvent,
    tile_object::{TileObjectIndex, TileObjectKind},
    world::World,
};

#[derive(PartialEq, Clone, Copy)]
pub enum Item {
    // Tools
    Tool(ItemTool),
    // Materials
    Resource(ItemResource),
}

impl Item {
    pub fn on_use(&self, target: TileObjectIndex, world: &mut World) -> Vec<WorldEvent> {
        match self {
            Item::Tool(tool) => tool.on_use(target, world),
            _ => vec![],
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum ItemTool {
    Hoe,
    Axe(Axe),
}
impl ItemTool {
    fn on_use(&self, target: TileObjectIndex, world: &mut World) -> Vec<WorldEvent> {
        match self {
            ItemTool::Axe(axe) => axe.on_use(target, world),
            _ => vec![],
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Axe {}
impl Axe {
    fn on_use(&self, target: TileObjectIndex, world: &mut World) -> Vec<WorldEvent> {
        let Some(tile_object) = world.get_mut_tile_object_from_index(target) else {
            return vec![];
        };

        match tile_object.kind {
            TileObjectKind::Tree(_) => vec![WorldEvent::TileObjectDamage {
                tile_object: target,
                damage: 1,
            }],
            _ => vec![],
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum ItemResource {
    Wood,
}

pub struct ItemStack {
    pub item: Item,
    pub count: u32,
}

pub struct WorldItemIndex(pub usize);

#[derive(PartialEq)]
pub struct WorldItem {
    pub item: Item,
    pub pos: Vec2,
}

impl Eq for WorldItem {}

impl PartialOrd for WorldItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.pos.y.partial_cmp(&other.pos.y)
    }
}

impl Ord for WorldItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
