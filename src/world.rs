use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
};

use bit_set::BitSet;
use macroquad::prelude::*;

use crate::{grid::Grid, item::ItemStack, map::Tile};

type EntityId = usize;

trait TileObjectStorage {
    type TileObjectComponent;

    fn get(&self, k: TileObject) -> Option<&Self::TileObjectComponent>;

    fn insert(&mut self, k: TileObject, v: Self::TileObjectComponent);
}

struct TileObjectHashMapStorage<T>(HashMap<TileObject, T>);
impl<T: TileObjectComponent> TileObjectStorage for TileObjectHashMapStorage<T> {
    type TileObjectComponent = T;

    fn get(&self, k: TileObject) -> Option<&T>
    where
        T: TileObjectComponent,
    {
        self.0.get(&k)
    }

    fn insert(&mut self, k: TileObject, v: Self::TileObjectComponent) {
        self.0.insert(k, v);
    }
}
impl<T> TileObjectHashMapStorage<T> {
    fn new() -> Self {
        TileObjectHashMapStorage(HashMap::new())
    }
}

pub struct World {
    // Tiles
    tiles: Grid<Tile>,

    // Tile objects
    tile_objects: Grid<TileObject>,
    tile_object_component_ids: HashMap<TileObject, HashSet<TypeId>>,
    tile_object_component_store: HashMap<TypeId, Box<dyn TileObjectStorage>>,
    tile_object_component_store_bitsets: HashMap<TypeId, BitSet>,
    tile_object_next_index: EntityId,
}

// Entity
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileObject(EntityId);

// Components
pub trait TileObjectComponent {}

pub struct Damageable {
    hp: u32,
}
impl TileObjectComponent for Damageable {}

pub struct Destroyable {
    drops: Vec<ItemStack>,
}
impl TileObjectComponent for Destroyable {}

// Systems
impl World {
    // command pattern
}

// Backing ECS data structures
impl World {
    pub fn create_tile_object(&mut self, pos: Vec2, components: Vec<Box<dyn TileObjectComponent>>) {
        let tile_object = TileObject(self.tile_object_next_index);
        self.tile_object_next_index += 1;

        let row = pos.y.round() as usize;
        let col = pos.x.round() as usize;
        self.tile_objects.set(row, col, tile_object);

        self.tile_object_component_ids.insert(
            tile_object,
            HashSet::from_iter(components.iter().map(|c| (*c).type_id())),
        );

        for component in components {
            let type_id = (*component).type_id();
            self.tile_object_component_store.entry(type_id);
            // .or_insert(Box::new(TileObjectHashMapStorage::new()))
            // .insert(tile_object, component);
            self.tile_object_component_store_bitsets
                .entry(type_id)
                .or_insert(BitSet::new())
                .insert(tile_object.0);
        }
    }
}
