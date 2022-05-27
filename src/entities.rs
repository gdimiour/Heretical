use rand::Rng;

pub struct Entity {
    entity: EntityType,
    position: Position,
    direction: f32,
    health: u16,
    hand: Item,
}

pub struct Position {pub x: f64, pub y: f64}

enum EntityType {
    Player,
}

struct Item {}

pub struct Animation {
    animation: AnimType,
    frame: usize,
}

enum AnimType {
    Default,
    Swing,
    Leap,
    Shoot,
    Stab,
}
