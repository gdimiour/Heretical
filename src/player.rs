use crate::*;
use entities::Animation;
use entities::Position;

pub struct Player {
    animstate: Animation,
    dir: f32,
    weapon: ItemType,
    position: Position,
    inventory: Vec<Item>,
}

struct Item {
    amount: u8,
    item: ItemType,
}

enum ItemType {
    Sword,
    Stick,
}