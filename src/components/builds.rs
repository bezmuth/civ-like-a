use amethyst::ecs::prelude::{DenseVecStorage, Component};
use super::tiles::TileType;




pub struct Build{
    pub mode: Option<TileType>,
}

pub struct Building{ // todo make these private?
    pub tile_type: TileType,
    pub playernum: i32,
    pub out_x: i32,
    pub out_y: i32,
}
impl Component for Building { // Component therefore use ReadStorage  an
    type Storage = DenseVecStorage<Self>;
}
