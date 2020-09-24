use crate::game::{Tiles, Layer2};

use amethyst::{
    ecs::prelude::{Join, System, WriteStorage},
    renderer::SpriteRender
};

use rand::prelude::*;

// * https://en.wikipedia.org/wiki/Perlin_noise for reference
// generates the 2d unit length gradient vectors
// todo: implement seeded random
fn gradientGen() -> [[[f32 ; 2];100];100]{ // todo: convert this to a map
    let mut gradientArray = [[[0. ; 2];100];100]; 
    for x in 0..100{
        for y in 0..100{
            let mut rng = rand::thread_rng();
            // todo: this might be a bit funky (v small x values) look into it a bit if terrain gets funky
            let gx = rng.gen::<f32>(); // here a random x value is genned, to get the y value of the unit vector
            let gy = (1.-(gx*gx)).sqrt();   // we perform the 1 = sqrt(x^2+y^2) rearranged
            gradientArray[x][y][0] = gx;
            gradientArray[x][y][1] = gy; 
            // println!("x: {} y: {}", gx, gy)
        }
    }
    return gradientArray;
}

// linear interpolates between a0 and a1 using weight w (w should be somewhere between 0.0 and 1.0)
fn lerp(a0: f32, a1: f32, w: f32) -> f32{
    // println!("a0: {}, a1: {}, w: {} ", a0, a1, w);
    return (1.0-w)*a0 + w*a1
}

// returns the dot product of the distance and gradient vectors
fn dotGridGradient(ix: i32, iy: i32, x : f32, y : f32, gradients : [[[f32 ; 2];100];100]) -> f32{ // iy and ix are the vectors at each node for each tile (4 nodes per tile)

    // calculate the distance between the point and the nodes
    let dx = x - ix as f32;
    let dy = y - iy as f32;

    // this bit calculates the dot product : https://en.wikipedia.org/wiki/Dot_product#Algebraic_definition
    return(dx*gradients[ix as usize][iy as usize][0] + dy* gradients[ix as usize][iy as usize][1])

}

fn perlin(x : f32, y: f32, gradients : [[[f32 ; 2];100];100]) -> f32{
    //convert into gradientarray coordinates
    let x0 = x as i32;
    let x1 = x0 +1;
    let y0 = y as i32;
    let y1 = y0+1;

    // calc lerp weights
    let sx = x - x0 as f32;
    let sy = y - y0 as f32;
    // lerp between grid point gradients
    let (n0, n1, ix0, ix1, value) : (f32, f32, f32, f32, f32);

    let n0 = dotGridGradient(x0, y0, x, y, gradients);
    let n1 = dotGridGradient(x1, y0, x, y, gradients);
    let ix0 = lerp(n0, n1, sx);

    let n0 = dotGridGradient(x0, y1, x, y, gradients);
    let n1 = dotGridGradient(x1, y1, x, y, gradients);
    let ix1 = lerp(n0, n1, 0.);

    let value = lerp(ix0, ix1, sy);
    // println!("{}", value);
    return value;

}



pub struct TerrainGenSystem{
    complete: bool,
    gradients: [[[f32 ; 2];100];100], //todo: calculate how big this should actually be
}

impl TerrainGenSystem{
    pub fn new() -> Self{
        TerrainGenSystem{complete:false, gradients: gradientGen()} // true if terrain has been genned
    }
}

impl<'s> System<'s> for TerrainGenSystem {
    type SystemData = (
        WriteStorage<'s, Tiles>,
        WriteStorage<'s, Layer2>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (mut tiles, layer2, mut spriterenderers): Self::SystemData) {
        if !self.complete{
            for (tile, spriterender, _) in (&mut tiles, &mut spriterenderers, & layer2).join(){
                if perlin(tile.x as f32 / 8., tile.y as f32 / 8. , self.gradients) < 0.13 { // divide by num to zoom into noise map
                    spriterender.sprite_number = 1 as usize;
                }
            }
        }
        self.complete = true;
    }


}