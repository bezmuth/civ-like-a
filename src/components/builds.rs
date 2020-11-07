use amethyst::ecs::prelude::{DenseVecStorage, Component};
use super::{unit::Unit, tiles::TileType};
// pub struct UnitStack{
//     top: i8,
//     arr: [Option<Unit>; 8],
//     pub repeat: bool,
// }

// impl UnitStack{
//     fn new() -> UnitStack{
//         UnitStack{top: 0, arr: [None; 8], repeat: false}
//     }

//     fn pop(&mut self) -> Option<Unit>{
//         if self.top == 0{
//             return None
//         } else {
//             if self.repeat{
//                 return self.arr[self.top as usize];
//             } else {
//                 self.top = self.top - 1;
//                 return self.arr[(self.top+1) as usize];
//             }
//         }
//     }

//     fn push(&mut self, unit : Unit) {
//         let unote = Some(unit);
//         self.arr[self.top as usize] = Some(unit);
//         self.top = self.top + 1;
//     }
// }



pub struct Build{
    pub mode: Option<TileType>,
}

pub struct Building{ // todo make these private?
    pub TileType: TileType,
    pub playernum: i32,
    pub out_x: i32,
    pub out_y: i32,
    // unit_stack: UnitStack, // todo: move into a seperate component
}
impl Component for Building { // Component therefore use ReadStorage  an
    type Storage = DenseVecStorage<Self>;
}