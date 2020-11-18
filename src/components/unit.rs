use amethyst::ecs::{Component, DenseVecStorage};
use crate::components::TileType;

#[derive(Copy, Clone)]
pub struct UnitStack{
    pub top: i8,
    arr: [Option<Unit>; 8],
    pub repeat: bool,
}
impl UnitStack{
    pub fn new() -> UnitStack{
        UnitStack{top: 0, arr: [None; 8], repeat: false}
    }

    pub fn pop(&mut self) -> Option<Unit>{
        if self.top == 0{
            return None
        } else {
            if self.repeat{
                return self.arr[self.top as usize];
            } else {
                self.top = self.top - 1;
                return self.arr[(self.top+1) as usize];
            }
        }
    }

    pub fn peek(&mut self) -> Option<Unit>{
        if self.top == 0{
            return None
        } else {
            if self.repeat{
                return self.arr[self.top as usize];
            } else {
                return self.arr[(self.top) as usize];
            }
        }
    }

    pub fn push(&mut self, unit : Unit) {
        self.arr[self.top as usize] = Some(unit);
        self.top = self.top + 1;
    }
}

impl Component for UnitStack{
    type Storage = DenseVecStorage<Self>;
}

#[derive(Copy, Clone)]
pub struct Unit{
    pub unit_type : TileType,
    pub health: i32,
}
impl Component for Unit {
    type Storage = DenseVecStorage<Self>;
}
