use amethyst::ecs::{Component, DenseVecStorage};
use crate::components::TileType;

#[derive(Copy, Clone)]
pub struct UnitStack{
    pub top: i8,
    arr: [Option<TileType>; 8],
    pub repeat: bool,
}
impl UnitStack{
    // okay yeah yeah I get it: "But Ben this is just a TileType stack, why call
    // it a unit stack when its just a TileType stack?" Okay, this was orginally
    // a "Unit" stack that just accepted the Unit type (which was just a
    // TileType wrapper), initally Unit was gonna contain all the stats, however
    // I split that off into a seperate struct, making it redundant to use
    // unitstack as a "Unit" stack, but I already had it used all over the code
    // and I did not want to change all of my code to "TileTypeStack" so "UnitStack" it
    // shall remain. Anyway, UnitStack kinda makes more sense because I only
    // ever use it for Units.
    pub fn new() -> UnitStack{
        UnitStack{top: 0, arr: [None; 8], repeat: false}
    }

    pub fn pop(&mut self) -> Option<TileType>{
        if self.top < 1{ // integer overflow avoidance and stops poping below 0
            return None
        } else {
            if self.repeat{
                return self.arr[self.top as usize];
            } else {
                println!("{}", self.top);
                self.top = self.top - 1;
                return self.arr[(self.top) as usize];
            }
        }
    }

    pub fn peek(&mut self) -> Option<TileType>{
        if self.top < 0{
            return None
        } else{
            return self.arr[(self.top) as usize];
        }
    }

    pub fn push(&mut self, tiletype : TileType) {
        if self.top != 8{
            println!("{}", self.top);
            self.arr[self.top as usize] = Some(tiletype);
            self.top += 1;
        }
    }
}

impl Component for UnitStack{
    type Storage = DenseVecStorage<Self>;
}

#[derive(Copy, Clone)]
pub struct Unit{
    pub unit_type : TileType,
    pub playernum: i32;
}
impl Component for Unit {
    type Storage = DenseVecStorage<Self>;
}
