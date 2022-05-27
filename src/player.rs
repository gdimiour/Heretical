use crate::*;
use entities::Animation;
use entities::Position;

pub struct Player {
    animstate: Animation,
    dir: f32,
    weapon: ItemType,
    pub position: Position,
    pub velocity: Position,
    inventory: Vec<Item>,
}

impl Player {
    pub fn left(player: &mut Self) {}
    pub fn middle(player: &mut Self) {}
    pub fn right(player: &mut Self) {}
    pub fn open(player: &mut Self) {}
    pub fn pickup(player: &mut Self) {}

}

pub struct Item {
    amount: u8,
    item: ItemType,
}

enum ItemType {
    Sword,
    Stick,
}