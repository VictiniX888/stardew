use std::collections::{BinaryHeap, HashMap};

use macroquad::prelude::*;

use crate::{
    game::GameState,
    grid::Grid,
    item::{Item, ItemStack, WorldItem},
    render::RenderState,
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
    pub objects: Grid<TileObject>,
    pub world_items: BinaryHeap<WorldItem>,
}

impl MapWorldData {
    fn load(tiled_map: tiled::Map) -> MapWorldData {
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

        let grid_objects = Grid::new(tiled_map.height as usize, tiled_map.width as usize);

        MapWorldData {
            tiles: grid,
            objects: grid_objects,
            world_items: BinaryHeap::new(),
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
    let world_data = MapWorldData::load(tiled_map);

    game_state.map_data = Some(world_data);
    game_state.player_pos = vec2(0.0, 0.0);

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
    EMPTY,
    GRASS,
    DIRT {
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
            Tile::DIRT { is_tilled } => *is_tilled = true,
            _ => (),
        }
    }
}

fn get_tile_from_id(tile_id: &str) -> Option<Tile> {
    Some(match tile_id {
        "farm_grass" => Tile::GRASS,
        "farm_dirt" => Tile::DIRT { is_tilled: false },
        _ => return None,
    })
}

#[derive(Default, Clone)]
pub enum TileObject {
    #[default]
    EMPTY,
    TREE {
        hp: u32,
    },
}

impl TileObject {
    pub fn get_drops(&self) -> Vec<ItemStack> {
        match self {
            TileObject::TREE { .. } => vec![ItemStack {
                item: Item::Wood,
                count: 5,
            }],
            _ => vec![],
        }
    }

    // Actions
    pub fn on_axe(mut self, game_state: &mut GameState) {
        match self {
            TileObject::TREE { mut hp } => {
                if hp > 0 {
                    hp -= 1
                } else {
                    self.on_destroy(game_state);
                }
            }
            _ => (),
        }
    }

    pub fn on_destroy(&mut self, game_state: &mut GameState) {
        *self = TileObject::EMPTY;
    }
}
