use rand::Rng;

pub struct Entity {
    entity: EntityType,
    position: Position,
    direction: f32,
    health: u16,
    hand: Item,
}

pub struct Position {x: f64, y: f64}

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
