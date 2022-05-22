use super::World;
use crate::*;

use entities::{Animation, Position};
use notan::prelude::keyboard::KeyCode;

use crate::screen::*;
use crate::tiles::Block;

impl World {

    pub fn create(gfx: &mut Graphics) -> World {
        World {chunks: HashMap::new(), loaded: todo!(), player: todo!() }
    }

    pub fn input_tick(app: &mut App, world: &mut Self) {
        let (x, y) = app.mouse.position();
        let player = world.player;
        /*for i in 0..5 {let mut n = self.chunks.get(self.loaded[i]).blocks}
        

        if app.mouse.left_is_down() {
            
            
        }
     
        if app.mouse.middle_is_down() {
            
        }
    
        if app.mouse.right_is_down() {
            
        }

        if app.keyboard.is_down(KeyCode::W) {}

        if app.keyboard.is_down(KeyCode::A) {}

        if app.keyboard.is_down(KeyCode::S) {}

        if app.keyboard.is_down(KeyCode::D) {}

        if app.keyboard.is_down(KeyCode::E) {}

        if app.keyboard.is_down(KeyCode::Space) {}*/
    }

    pub fn output_tick(app: &mut App, gfx: &mut Graphics, world: &mut Self) {
        let mut draw = gfx.create_draw();
        draw.clear(Color::BLACK);
        let mut blocks: Vec<Block> = Vec::new();
        for i in &world.loaded {
            for j in &mut world.chunks.get(i).unwrap().blocks{
                blocks.append(vec![j])
            }
        }
        renderblocks(blocks , &mut draw);
        gfx.render(&draw)
    }

}

pub struct Player {
    animstate: Animation,
    dir: f32,
    weapon: usize,
    x: f64,
    y: f64,

}
