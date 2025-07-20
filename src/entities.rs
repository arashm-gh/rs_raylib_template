use raylib::prelude::*;
pub enum EntityType {
    Player,
    Wall,
    Floor,
}
pub struct Entity {
    pos: Vector2,
    e_type: EntityType,
    texture: Option<Texture2D>,
}

impl Entity {
    pub fn new(pos:Vector2, e_type:EntityType) -> Self {
        Entity {
            pos,
            e_type,
            texture: None,
        }
    }
}
