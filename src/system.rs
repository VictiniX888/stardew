use crate::{game::GameState, item::WorldItemIndex};

pub fn pickup_world_items(game_state: &mut GameState) {
    let player = &mut game_state.player;
    let world = game_state.world.as_mut().unwrap();

    let nearby_items = world
        .world_items
        .iter()
        .enumerate()
        .filter(|(_, item)| player.pos.distance(item.pos) < 0.5)
        .map(|(idx, _)| WorldItemIndex(idx))
        .collect::<Vec<_>>();

    for item in nearby_items {
        player.pickup_world_item(world, item);
    }
}
