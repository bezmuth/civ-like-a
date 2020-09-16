use amethyst::ecs::prelude::{DenseVecStorage, Component};
#[derive(Copy, Clone, PartialEq)]
pub enum BuildingType{ // resource as it has no component implmentation, use READ and READEXPECT
    Center = 3,
    WarBuilding = 4,
    WoodBuilding = 5,
    MetalBuilding = 6,
    FaithBuilding = 7,
    Demolish = 99,
}

pub struct Build{
    pub mode: Option<BuildingType>,
}

pub struct Building{ 
    pub buildingtype: BuildingType,
    pub playernum: i32,
    pub x: i32,
    pub y: i32,
}
impl Component for Building { // Component therefore use ReadStorage  an
    type Storage = DenseVecStorage<Self>;
}