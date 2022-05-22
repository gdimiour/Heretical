pub struct Entity {
    entity: EntityType,
    position: Position,
    direction: f32,
    health: u16,
    data: Vec<Data>,
}

pub struct Position {x: f64, y: f64}

enum Data { 
    Inventory([Item; 20]),
    Hands([Item; 2]),
    Armor([Item; 8]),
}

enum EntityType {
    Player,
}

struct Item {}

pub struct Animation {
    animation: AnimType,
    frame: usize,
}

enum AnimType {
    Swing,
    Leap,
    Shoot,
    Stab,
}
