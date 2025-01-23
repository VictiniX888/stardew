use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    game::GameState,
    item::{Item, ItemResource, WorldItem},
    map::{MapRenderData, TextureMap, Tile},
    tile_object::{TileObject, TileObjectKind, TileObjectTreeKind},
};

pub struct RenderState {
    pub map_data: Option<MapRenderData>,
    pub player_texture: Option<Texture2D>,

    pub zoom: i32,
}

impl RenderState {
    pub fn init() -> RenderState {
        RenderState {
            map_data: None,
            player_texture: None,
            zoom: 1,
        }
    }

    pub fn render_world(&self, game_state: &GameState) {
        if self.map_data.is_none() || game_state.map_data.is_none() || game_state.world.is_none() {
            panic!("No world to render");
        }
        let map_render_data = self.map_data.as_ref().unwrap();
        let map_world_data = game_state.map_data.as_ref().unwrap();
        let world = game_state.world.as_ref().unwrap();

        let tile_w = (16 * self.zoom) as f32;

        // Draw map
        for ((row, col), tile) in map_world_data.tiles.enumerate_iter() {
            let tex = get_tile_texture(tile, &map_render_data.texturemap);
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
        let Some(tex) =
            get_tile_object_texture(tile_object, &self.map_data.as_ref().unwrap().texturemap)
        else {
            return;
        };
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
        let Some(tex) = get_item_texture(&item.item, &self.map_data.as_ref().unwrap().texturemap)
        else {
            return;
        };
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

    pub async fn load_player_texture(&mut self) {
        let tex = load_texture("tiled/characters/character_cat.png")
            .await
            .unwrap();
        tex.set_filter(FilterMode::Nearest);
        self.player_texture = Some(tex);
    }

    pub async fn load_tile_object_textures(&mut self) {
        assert!(self.map_data.is_some());

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
            .as_mut()
            .unwrap()
            .texturemap
            .insert("objects".to_string(), obj_texs);
    }

    pub async fn load_item_textures(&mut self) {
        assert!(self.map_data.is_some());

        let mut item_texs = HashMap::new();
        let tex = load_texture("tiled/items/item_wood.png").await.unwrap();
        tex.set_filter(FilterMode::Nearest);
        item_texs.insert(0, tex);

        self.map_data
            .as_mut()
            .unwrap()
            .texturemap
            .insert("items".to_string(), item_texs);
    }
}

enum RenderObject {
    TileObject,
    WorldItem,
    PlayerCharacter,
}

fn get_tile_texture<'a>(tile: &Tile, texturemap: &'a TextureMap) -> &'a Texture2D {
    match tile {
        Tile::Grass => texturemap.get("farm").unwrap().get(&0).unwrap(),
        Tile::Dirt { is_tilled: false } => texturemap.get("farm").unwrap().get(&1).unwrap(),
        Tile::Dirt { is_tilled: true } => texturemap.get("farm").unwrap().get(&2).unwrap(),
        _ => panic!("No valid texture for tile"),
    }
}

fn get_tile_object_texture<'a>(
    tile_object: &TileObject,
    texturemap: &'a TextureMap,
) -> Option<&'a Texture2D> {
    Some(match &tile_object.kind {
        TileObjectKind::Tree(tree) => match tree.kind {
            TileObjectTreeKind::TreeFullyGrown(_) => {
                texturemap.get("objects").unwrap().get(&0).unwrap()
            }
            TileObjectTreeKind::TreeStump(_) => texturemap.get("objects").unwrap().get(&1).unwrap(),
        },
    })
}

fn get_item_texture<'a>(item: &Item, texturemap: &'a TextureMap) -> Option<&'a Texture2D> {
    Some(match item {
        Item::Resource(item) => match item {
            ItemResource::Wood => texturemap.get("items").unwrap().get(&0).unwrap(),
        },
        _ => panic!("No valid texture for item"),
    })
}
