use macroquad::prelude::*;

use crate::{
    grid::Grid,
    item::{ItemStack, WorldItem, WorldItemIndex},
    tile_object::{TileObject, TileObjectIndex},
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

    pub fn get_world_item(&self, index: WorldItemIndex) -> &WorldItem {
        &self.world_items[index.0]
    }

    pub fn remove_world_item(&mut self, index: WorldItemIndex) -> WorldItem {
        self.world_items.remove(index.0)
    }
}
