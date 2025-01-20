use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    game::GameState,
    map::{MapRenderData, TextureMap, Tile, TileObject},
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
        if self.map_data.is_none() || game_state.map_data.is_none() {
            panic!("No world to render");
        }
        let map_render_data = self.map_data.as_ref().unwrap();
        let map_world_data = game_state.map_data.as_ref().unwrap();

        let tile_w = (16 * self.zoom) as f32;

        // Draw map
        for ((row, col), tile) in map_world_data.tiles.enumerate_iter() {
            let tex = get_tile_texture(tile, &map_render_data.texturemap);
            draw_texture_ex(
                tex,
                (col as f32 - game_state.player_pos.x) * tile_w + screen_width() / 2.0
                    - tile_w / 2.0,
                (row as f32 - game_state.player_pos.y) * tile_w + screen_height() / 2.0
                    - tile_w / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(tile_w as f32, tile_w as f32)),
                    ..Default::default()
                },
            );
        }

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
        let tex = load_texture("tiled/tiles/farm_tree.png").await.unwrap();
        tex.set_filter(FilterMode::Nearest);
        obj_texs.insert(0, tex);

        self.map_data
            .as_mut()
            .unwrap()
            .texturemap
            .insert("objects".to_string(), obj_texs);
    }
}

fn get_tile_texture<'a>(tile: &Tile, texturemap: &'a TextureMap) -> &'a Texture2D {
    match tile {
        Tile::GRASS => texturemap.get("farm").unwrap().get(&0).unwrap(),
        Tile::DIRT { is_tilled: false } => texturemap.get("farm").unwrap().get(&1).unwrap(),
        Tile::DIRT { is_tilled: true } => texturemap.get("farm").unwrap().get(&2).unwrap(),
        _ => panic!("No valid texture for tile"),
    }
}

fn get_tile_object_texture<'a>(
    tile_object: &TileObject,
    texturemap: &'a TextureMap,
) -> Option<&'a Texture2D> {
    Some(match tile_object {
        TileObject::TREE { .. } => texturemap.get("objects").unwrap().get(&0).unwrap(),
        _ => return None,
    })
}
