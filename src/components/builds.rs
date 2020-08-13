use amethyst::ecs::prelude::{DenseVecStorage, Component};
#[derive(Copy, Clone)]
pub enum BuildingType{ // resource as it has no component implmentation, use READ and READEXPECT
    Center,
    WarBuilding,
    WoodBuilding,
    MetalBuilding,
    FaithBuilding,
    None,
}

pub struct Build{
    pub mode: BuildingType,
}

pub struct Building{ 
    pub buildingtype: BuildingType,
    pub playernum: i8,
}
impl Component for Building { // Component therefore use ReadStorage  an
    type Storage = DenseVecStorage<Self>;
}