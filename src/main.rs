use notan::draw::*;
use notan::prelude::*;
pub use std::collections::HashMap;

mod world;
mod chunks;
mod entities;
mod tiles;
mod screen;

use world::Player;
use chunks::Chunk;

#[derive(AppState)]
struct World {
    chunks: HashMap<(i64, i64),Chunk>,
    loaded: [(i64, i64); 25],
    player: Player,
}

fn main() {
    notan::init_with(World::create)
        .add_config(WindowConfig::new().vsync())
        .add_config(DrawConfig)
        .update(World::input_tick)
        .draw(World::output_tick)
        .build();
}
//ello this is cool monke 27 test #2