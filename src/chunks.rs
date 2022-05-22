use super::World;
use crate::*;
use rand::Rng;

use entities::Entity;
use tiles::Block;

pub struct Chunk {
    pub blocks: [Block; 1024],
    pub background: [Color; 1024],
    pub entities: Vec<Entity>,
    pub x: u64,
    pub y: u64,
}

impl Chunk {
    pub fn loadnew(x: i64, y: i64) /*-> Self*/ {
        
        
        /*Self {
            entities: vec![],
        }*/
    } 

    pub fn loadold(strdata: &mut str) /*-> Self*/ {

    }

    pub fn chunktick() {}
}

/*
Chunks:

[ block mouse is on , mouse down? , chunk rendered? ] -> [ chunks , rendered chunks ]
*/