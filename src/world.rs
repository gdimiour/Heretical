use super::World;
use crate::*;

use entities::{Animation, Position};
use notan::prelude::keyboard::KeyCode;
use player::Player;

use crate::screen::*;
use crate::tiles::Block;

impl World {

    pub fn create(gfx: &mut Graphics) -> World {

        World {chunks: HashMap::new(), loaded: todo!(), player: todo!(), assets: todo!()}
    }

    /*pub fn input_tick(app: &mut App, world: &mut Self) {
        let (x, y) = app.mouse.velocity();
        let player = world.player;
        for i in 0..5 {let mut n = self.chunks.get(self.loaded[i]).blocks}
        

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

        if app.keyboard.is_down(KeyCode::Space) {}
    }*/

    pub fn tick(app: &mut App, gfx: &mut Graphics, world: &mut Self) {
        
        let mut draw = gfx.create_draw();
        draw.clear(Color::BLACK);

        for chunk in &world.loaded {
            for block in &world.chunks.get(chunk).unwrap().blocks{
                renderblock(block, &mut draw, &world.assets, &world.player.velocity);
            }
        }

        app.window().set_cursor(CursorIcon::None);
        draw.image(&world.assets[0]).position(app.mouse.x, app.mouse.y);
        gfx.render(&draw);
        
        let (x, y) = app.mouse.position();
        let player = &world.player;

        for i in 0..5 {let mut _n = &world.chunks.get(&world.loaded[i]).unwrap().blocks;}

        if app.mouse.left_is_down() {/*world.player.middle()*/}
     
        if app.mouse.middle_is_down() {/*world.player.middle()*/}
    
        if app.mouse.right_is_down() {/*world.player.middle()*/}

        if app.keyboard.is_down(KeyCode::W) {world.player.velocity.y -= 0.2;}

        if app.keyboard.is_down(KeyCode::A) {world.player.velocity.x -= 0.2;}

        if app.keyboard.is_down(KeyCode::S) {world.player.velocity.y += 0.2;}

        if app.keyboard.is_down(KeyCode::D) {world.player.velocity.x += 0.2;}

        if app.keyboard.is_down(KeyCode::E) {/*world.player.open()*/}

        if app.keyboard.is_down(KeyCode::Space) {}
        
        if world.player.velocity.x != 0.0 {world.player.velocity.x /= 1.2;}

        if world.player.velocity.y != 0.0 {world.player.velocity.y /= 1.2;}
    }

}

