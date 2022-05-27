use notan::prelude::*;
use notan::draw::*;
use crate::Draw;

use crate::tiles::Block;
use crate::player::Player;
use crate::entities::Entity;
use crate::entities::Position;

pub fn renderblock(renderblocks: &Block, draw: &mut Draw, assets: &[Texture; 1], position: &Position) {
    draw.image(&assets[0]).position(250.0, 200.0);
}

pub fn renderentity(renderblocks: Entity, draw: &mut Draw) {

}