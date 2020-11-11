use amethyst::ecs::{Component, DenseVecStorage};

pub struct UnitStack{
    top: i8,
    arr: [Option<Unit>; 8],
    pub repeat: bool,
}

impl UnitStack{
    fn new() -> UnitStack{
        UnitStack{top: 0, arr: [None; 8], repeat: false}
    }

    fn pop(&mut self) -> Option<Unit>{
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

    fn push(&mut self, unit : Unit) {
        let unote = Some(unit);
        self.arr[self.top as usize] = Some(unit);
        self.top = self.top + 1;
    }
}

impl Component for UnitStack{
    type Storage = DenseVecStorage<Self>;
}

#[derive(Copy, Clone)]
pub struct Unit{
    
}
impl Component for Unit {
    type Storage = DenseVecStorage<Self>;
}
