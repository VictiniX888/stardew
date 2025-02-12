use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    game::GameState,
    item::{Item, ItemResource, WorldItem},
    map::{MapRenderData, Tile},
    tile_object::{TileObject, TileObjectKind, TileObjectTreeKind},
};

pub struct RenderState {
    pub map_data: MapRenderData,
    pub player_texture: Option<Texture2D>,

    pub zoom: u32,
}

impl RenderState {
    pub fn init() -> RenderState {
        RenderState {
            map_data: MapRenderData::new(),
            player_texture: None,
            zoom: 1,
        }
    }

    pub fn render_world(&self, game_state: &GameState) {
        if game_state.map_data.is_none() || game_state.world.is_none() {
            panic!("No world to render");
        }
        let map_world_data = game_state.map_data.as_ref().unwrap();
        let world = game_state.world.as_ref().unwrap();

        let tile_w = (16 * self.zoom) as f32;

        // Draw map
        for ((row, col), tile) in map_world_data.tiles.enumerate_iter() {
            let tex = self.get_tile_texture(tile);
            draw_texture_ex(
                tex,
                (col as f32 - game_state.player.pos.x) * tile_w + screen_width() / 2.0
                    - tile_w / 2.0,
                (row as f32 - game_state.player.pos.y) * tile_w + screen_height() / 2.0
                    - tile_w / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(tile_w as f32, tile_w as f32)),
                    ..Default::default()
                },
            );
        }

        // Draw world objects with sprite sorting (tile objects, world items, characters)
        let mut iter_tile_objects = world.tile_objects.enumerate_iter_sparse();
        let mut iter_world_items = world.world_items.iter();
        let mut next_tile_object: Option<((usize, usize), &TileObject)> = iter_tile_objects.next();
        let mut next_world_item: Option<&WorldItem> = iter_world_items.next();
        let mut next_player_character = Some(game_state.player.pos);

        while next_tile_object.is_some()
            || next_world_item.is_some()
            || next_player_character.is_some()
        {
            let mut render_object = None;
            let mut min_y = None;

            // Compare y order
            if let Some(((row, _), _)) = next_tile_object {
                render_object = Some(RenderObject::TileObject);
                min_y = Some(row as f32);
            }

            if let Some(world_item) = next_world_item {
                if min_y.is_none() || world_item.pos.y < min_y.unwrap() {
                    render_object = Some(RenderObject::WorldItem);
                    min_y = Some(world_item.pos.y);
                }
            }

            if let Some(pos) = next_player_character {
                if min_y.is_none() || pos.y < min_y.unwrap() {
                    render_object = Some(RenderObject::PlayerCharacter);
                    // min_y = Some(pos.y);
                }
            }

            assert!(render_object.is_some());

            // Render
            let Some(render_object) = render_object else {
                break;
            };
            match render_object {
                RenderObject::TileObject => {
                    let ((row, col), tile_object) = next_tile_object.unwrap();
                    self.render_tile_object(row, col, tile_object, tile_w, game_state);
                }
                RenderObject::WorldItem => {
                    self.render_world_item(next_world_item.unwrap(), tile_w, game_state);
                }

                RenderObject::PlayerCharacter => {
                    self.render_player_character(tile_w, game_state);
                }
            }

            // Advance iterator
            match render_object {
                RenderObject::TileObject => next_tile_object = iter_tile_objects.next(),
                RenderObject::WorldItem => next_world_item = iter_world_items.next(),
                RenderObject::PlayerCharacter => next_player_character = None,
            }
        }
    }

    fn render_tile_object(
        &self,
        row: usize,
        col: usize,
        tile_object: &TileObject,
        tile_w: f32,
        game_state: &GameState,
    ) {
        let tex = self.get_tile_object_texture(tile_object);
        let tex_w = tex.width() * self.zoom as f32;
        let tex_h = tex.height() * self.zoom as f32;
        draw_texture_ex(
            tex,
            (col as f32 - game_state.player.pos.x) * tile_w + screen_width() / 2.0 - tile_w / 2.0,
            (row as f32 - game_state.player.pos.y) * tile_w + screen_height() / 2.0 + tile_w / 2.0
                - tex_h,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(tex_w, tex_h)),
                ..Default::default()
            },
        );
    }

    fn render_world_item(&self, item: &WorldItem, tile_w: f32, game_state: &GameState) {
        let tex = self.get_item_texture(&item.item);
        let item_w = tile_w * 0.8;
        draw_texture_ex(
            tex,
            (item.pos.x - game_state.player.pos.x) * tile_w + screen_width() / 2.0 - item_w / 2.0,
            (item.pos.y - game_state.player.pos.y) * tile_w + screen_height() / 2.0 - item_w / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(item_w as f32, item_w as f32)),
                ..Default::default()
            },
        );
    }

    fn render_player_character(&self, tile_w: f32, _game_state: &GameState) {
        // Draw player
        if let Some(player_texture) = &self.player_texture {
            draw_texture_ex(
                player_texture,
                screen_width() / 2.0 - tile_w / 2.0,
                screen_height() / 2.0 - tile_w / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(tile_w as f32, tile_w as f32)),
                    ..Default::default()
                },
            );
        }
    }

    pub fn render_hotbar(&self, game_state: &GameState) {
        const HOTBAR_SLOTS: u32 = 10;
        const SLOT_WIDTH: f32 = 32.0;
        const HOTBAR_BORDER: f32 = 4.0;
        const HOTBAR_WIDTH: f32 =
            (SLOT_WIDTH + HOTBAR_BORDER) * HOTBAR_SLOTS as f32 + HOTBAR_BORDER;
        const HOTBAR_HEIGHT: f32 = SLOT_WIDTH + HOTBAR_BORDER * 2.0;

        // Draw hotbar outline
        let screen_width = screen_width();
        let screen_height = screen_height();
        let left = (screen_width - HOTBAR_WIDTH) / 2.0;
        let top = screen_height - HOTBAR_HEIGHT;
        draw_rectangle(left, top, HOTBAR_WIDTH, HOTBAR_HEIGHT, DARKGRAY);
        draw_rectangle_lines(left, top, HOTBAR_WIDTH, HOTBAR_HEIGHT, HOTBAR_BORDER, WHITE);
        for slot in 1..HOTBAR_SLOTS {
            // Draw internal borders
            let x = left + (SLOT_WIDTH + HOTBAR_BORDER) * slot as f32 + HOTBAR_BORDER / 2.0;
            draw_line(x, top, x, screen_height, HOTBAR_BORDER / 2.0, WHITE);
        }

        // Render items
        for (slot, itemstack) in game_state.player.inventory.iter().enumerate() {
            let slot_left = left + (SLOT_WIDTH + HOTBAR_BORDER) * slot as f32 + HOTBAR_BORDER;
            let slot_right =
                left + (SLOT_WIDTH + HOTBAR_BORDER) * (slot + 1) as f32 - HOTBAR_BORDER / 2.0;
            let slot_top = top + HOTBAR_BORDER;
            let slot_bottom = top + SLOT_WIDTH + HOTBAR_BORDER;
            draw_texture_ex(
                self.get_item_texture(&itemstack.item),
                slot_left,
                slot_top,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(SLOT_WIDTH, SLOT_WIDTH)),
                    ..Default::default()
                },
            );
            let text_dim = measure_text(&itemstack.count.to_string(), None, 15, 1.0);
            draw_text(
                &itemstack.count.to_string(),
                slot_right - text_dim.width,
                slot_bottom - text_dim.height + text_dim.offset_y,
                15.0,
                WHITE,
            );
        }
    }

    pub async fn load_player_texture(&mut self) {
        let tex = load_texture("tiled/characters/character_cat.png")
            .await
            .unwrap();
        tex.set_filter(FilterMode::Nearest);
        self.player_texture = Some(tex);
    }

    pub async fn load_tile_object_textures(&mut self) {
        let mut obj_texs = HashMap::new();

        let tex = load_texture("tiled/tile_objects/tree_fully_grown.png")
            .await
            .unwrap();
        tex.set_filter(FilterMode::Nearest);
        obj_texs.insert(0, tex);

        let tex = load_texture("tiled/tile_objects/tree_stump.png")
            .await
            .unwrap();
        tex.set_filter(FilterMode::Nearest);
        obj_texs.insert(1, tex);

        self.map_data
            .texturemap
            .insert("objects".to_string(), obj_texs);
    }

    pub async fn load_item_textures(&mut self) {
        let mut item_texs = HashMap::new();
        let tex = load_texture("tiled/items/item_wood.png").await.unwrap();
        tex.set_filter(FilterMode::Nearest);
        item_texs.insert(0, tex);

        self.map_data
            .texturemap
            .insert("items".to_string(), item_texs);
    }

    fn get_tile_texture(&self, tile: &Tile) -> &Texture2D {
        match tile {
            Tile::Grass => self
                .map_data
                .texturemap
                .get("farm")
                .unwrap()
                .get(&0)
                .unwrap(),
            Tile::Dirt { is_tilled: false } => self
                .map_data
                .texturemap
                .get("farm")
                .unwrap()
                .get(&1)
                .unwrap(),
            Tile::Dirt { is_tilled: true } => self
                .map_data
                .texturemap
                .get("farm")
                .unwrap()
                .get(&2)
                .unwrap(),
            _ => panic!("No valid texture for tile"),
        }
    }

    fn get_tile_object_texture(&self, tile_object: &TileObject) -> &Texture2D {
        match &tile_object.kind {
            TileObjectKind::Tree(tree) => match tree.kind {
                TileObjectTreeKind::TreeFullyGrown(_) => self
                    .map_data
                    .texturemap
                    .get("objects")
                    .unwrap()
                    .get(&0)
                    .unwrap(),
                TileObjectTreeKind::TreeStump(_) => self
                    .map_data
                    .texturemap
                    .get("objects")
                    .unwrap()
                    .get(&1)
                    .unwrap(),
            },
        }
    }

    fn get_item_texture(&self, item: &Item) -> &Texture2D {
        match item {
            Item::Resource(item) => match item {
                ItemResource::Wood => self
                    .map_data
                    .texturemap
                    .get("items")
                    .unwrap()
                    .get(&0)
                    .unwrap(),
            },
            _ => panic!("No valid texture for item"),
        }
    }
}

enum RenderObject {
    TileObject,
    WorldItem,
    PlayerCharacter,
}
