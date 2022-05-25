use notan::draw::*;
use notan::prelude::*;
pub use std::collections::HashMap;

mod world; // World ticks and initiation
mod chunks; //where blocks/entities are stored
mod entities; //entity type and other
mod tiles; //block and ground
mod screen; //renders you, the blocks, and the entities
mod player; //you, your inventory and other data

use player::Player;
use chunks::Chunk;

#[derive(AppState)]
struct World {
    chunks: HashMap<(i64, i64),Chunk>, // Where all the data is stored
    loaded: [(i64, i64); 25],
    player: Player,
    assets: [Texture; 1],
}

fn main() {
    notan::init_with(World::create) //This is the initiation. Put code you want to run at start in create world function.
        .add_config(WindowConfig::new().vsync())
        .add_config(DrawConfig)
        //.update(World::input_tick)
        .draw(World::/*output_*/tick)
        .build();
}

//repo: https://github.com/xDimi/Heretical