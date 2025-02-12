use macroquad::math::{vec2, Vec2};

use crate::{
    item::{Item, ItemResource, ItemStack, WorldItemIndex},
    world::World,
};

pub struct Player {
    pub pos: Vec2,
    pub inventory: Inventory,
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos: vec2(0.0, 0.0),
            inventory: Inventory::new(),
        }
    }

    pub fn pickup_world_item(&mut self, world: &mut World, index: WorldItemIndex) {
        let world_item = world.remove_world_item(index);
        self.inventory.add_items(ItemStack {
            item: world_item.item,
            count: 1,
        });
    }
}

pub struct Inventory {
    items: Vec<ItemStack>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory { items: Vec::new() }
    }

    pub fn add_items(&mut self, items: ItemStack) {
        match self.items.iter_mut().find(|it| it.item == items.item) {
            Some(it) => it.count += items.count,
            None => self.items.push(items),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, ItemStack> {
        self.items.iter()
    }
}
