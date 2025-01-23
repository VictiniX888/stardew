use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    game::GameState, grid::Grid, render::RenderState, tile_object::TreeFullyGrown, world::World,
};

pub enum MapType {
    Farm,
}

impl MapType {
    const fn get_tiled_path(&self) -> &'static str {
        match self {
            MapType::Farm => "tiled/farm.tmx",
        }
    }
}

pub type TextureMap = HashMap<String, HashMap<u32, Texture2D>>;

pub struct MapWorldData {
    pub tiles: Grid<Tile>,
}

impl MapWorldData {
    fn load(tiled_map: &tiled::Map) -> MapWorldData {
        let mut grid = Grid::new(tiled_map.height as usize, tiled_map.width as usize);
        for layer in tiled_map.layers() {
            if let Some(layer) = layer.as_tile_layer() {
                let width = layer.width().unwrap() as usize;
                let height = layer.height().unwrap() as usize;
                for row in 0..height {
                    for col in 0..width {
                        if let Some(tile) = layer
                            .get_tile(col as i32, row as i32)
                            .and_then(|tile| tile.get_tile())
                        {
                            if let Some(tile) = Tile::from_tiled_tile(&tile) {
                                grid.set(row, col, tile);
                            } else {
                                panic!("Tile at {row},{col} had no or invalid properties");
                            }
                        }
                    }
                }
            }
        }

        MapWorldData { tiles: grid }
    }

    fn load_world(tiled_map: &tiled::Map) -> World {
        let width = tiled_map.width as usize;
        let height = tiled_map.height as usize;
        let mut tile_objects = Grid::new(height, width);

        // Randomly generate trees
        for row in 0..height {
            for col in 0..width {
                if rand::gen_range(0, 100) == 0 {
                    tile_objects.set(row, col, Some(TreeFullyGrown::new()));
                }
            }
        }

        let world_items = Vec::new();

        World {
            tile_objects,
            world_items,
        }
    }
}

pub struct MapRenderData {
    pub texturemap: TextureMap,
}

impl MapRenderData {
    async fn load(tiled_map: &tiled::Map) -> MapRenderData {
        let texturemap = init_tiles_for_render(tiled_map).await;
        MapRenderData { texturemap }
    }
}

pub async fn load_map(map: MapType, game_state: &mut GameState, render_state: &mut RenderState) {
    let tiled_map = load_tiled_map_from_path(map.get_tiled_path());

    let render_data = MapRenderData::load(&tiled_map).await;
    let map_world_data = MapWorldData::load(&tiled_map);
    let world_data = MapWorldData::load_world(&tiled_map);

    game_state.map_data = Some(map_world_data);
    game_state.player.pos = vec2(0.0, 0.0);
    game_state.world = Some(world_data);

    render_state.map_data = Some(render_data);
}

fn load_tiled_map_from_path(path: &str) -> tiled::Map {
    let mut loader = tiled::Loader::new();
    let map = loader.load_tmx_map(path).unwrap();
    map
}

async fn init_tiles_for_render(map: &tiled::Map) -> TextureMap {
    let mut tilesets = HashMap::with_capacity(map.tilesets().len());
    for tileset in map.tilesets() {
        let mut tilemap = HashMap::with_capacity(tileset.tilecount as usize);
        for (tid, tile) in tileset.tiles() {
            let tex = load_texture(tile.image.as_ref().unwrap().source.to_str().unwrap())
                .await
                .unwrap();
            tex.set_filter(FilterMode::Nearest);
            tilemap.insert(tid, tex);
        }
        tilesets.insert(tileset.name.clone(), tilemap);
    }

    tilesets
}

#[derive(Default, Clone)]
pub enum Tile {
    #[default]
    Empty,
    Grass,
    Dirt {
        is_tilled: bool,
    },
}

impl Tile {
    pub fn from_tiled_tile(tile: &tiled::Tile) -> Option<Tile> {
        let id = tile.properties.get("id")?;
        if let tiled::PropertyValue::StringValue(id) = id {
            get_tile_from_id(id.as_str())
        } else {
            None
        }
    }

    // Actions
    pub fn on_hoe(&mut self) {
        match self {
            Tile::Dirt { is_tilled } => *is_tilled = true,
            _ => (),
        }
    }
}

fn get_tile_from_id(tile_id: &str) -> Option<Tile> {
    Some(match tile_id {
        "farm_grass" => Tile::Grass,
        "farm_dirt" => Tile::Dirt { is_tilled: false },
        _ => return None,
    })
}
