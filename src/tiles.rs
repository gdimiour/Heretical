
pub struct Block {
    pub blocktype: BlockType,
    pub health: f32,
}

impl Block {
    pub fn damage(&mut self, amount: f32) -> bool {
        self.health -= amount;
        if self.health <= 0.0 {true} else {false}
    }

    pub fn new(blocktype: BlockType) -> Self{
        Self {health: match blocktype {
            BlockType::Gold => 100.0
        }, blocktype: blocktype}
    }
}


pub enum BlockType {
    Gold,
}