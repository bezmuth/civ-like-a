use amethyst::ecs::prelude::{DenseVecStorage, Component};
use super::TilePos;
use super::tiles::TileType;


pub struct OutPos{
    pub pos : TilePos, 
}
impl Component for OutPos{
    type Storage = DenseVecStorage<Self>;
}

pub struct Build{
    pub mode: Option<TileType>,
}

pub struct Building{ // todo make these private?
    pub tile_type: TileType,
    pub playernum: i32,
}
impl Component for Building { // Component therefore use ReadStorage  an
    type Storage = DenseVecStorage<Self>;
}
