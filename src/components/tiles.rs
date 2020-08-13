use amethyst::ecs::prelude::{DenseVecStorage, Component};
use super::BuildingType;
pub struct Tiles{ 
    pub layer: i8,
    pub player: i8,
    pub buildingtype: BuildingType,
    pub x: i32,
    pub y: i32,
}
impl Component for Tiles { // Component therefore use ReadStorage  an
    type Storage = DenseVecStorage<Self>;
}