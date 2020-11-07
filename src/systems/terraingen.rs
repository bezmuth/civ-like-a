use crate::game::{Layer1, TilePos, Tiles};

use amethyst::{
    ecs::prelude::{Join, System, WriteStorage},
    renderer::SpriteRender
};

use rand::prelude::*;

// * https://en.wikipedia.org/wiki/Perlin_noise for reference
// generates the 2d unit length gradient vectors
// todo: implement seeded random
fn gradient_gen() -> [[[f32 ; 2];100];100]{ // todo: convert this to a map
    let mut gradient_array = [[[0. ; 2];100];100]; 
    for x in 0..100{
        for y in 0..100{
            let mut rng = rand::thread_rng();
            // todo: this might be a bit funky (v small x values) look into it a bit if terrain gets funky
            let gx = rng.gen::<f32>(); // here a random x value is genned, to get the y value of the unit vector
            let gy = (1.-(gx*gx)).sqrt();   // we perform the 1 = sqrt(x^2+y^2) rearranged
            gradient_array[x][y][0] = gx;
            gradient_array[x][y][1] = gy; 
            // println!("x: {} y: {}", gx, gy)
        }
    }
    return gradient_array;
}

// linear interpolates between a0 and a1 using weight w (w should be somewhere between 0.0 and 1.0)
fn lerp(a0: f32, a1: f32, w: f32) -> f32{ // http://paulbourke.net/miscellaneous/interpolation/ to explain
    // println!("a0: {}, a1: {}, w: {} ", a0, a1, w);
    return (1.0-w)*a0 + w*a1
}

// returns the dot product of the distance and gradient vectors
fn dot_grid_gradient(ix: i32, iy: i32, x : f32, y : f32, gradients : [[[f32 ; 2];100];100]) -> f32{ // iy and ix are the vectors at each node for each tile (4 nodes per tile)

    // calculate the distance between the point and the nodes
    let dx = x - ix as f32; // For each gradient, we calculate the offset vector from the corner to the candidate point. We take the dot product between each pair of gradient vector and offset vector. This value will be zero if the candidate point is exactly at a grid corner. 
    let dy = y - iy as f32;

    // this bit calculates the dot product : https://en.wikipedia.org/wiki/Dot_product#Algebraic_definition
    return dx*gradients[ix as usize][iy as usize][0] + dy* gradients[ix as usize][iy as usize][1]

}

fn perlin(x : f32, y: f32, gradients : [[[f32 ; 2];100];100]) -> f32{

    let xedge = x.floor() as i32;
    let yedge = y.floor() as i32;

    // the other edge gradients have to be used so we can interpolate the noise between the two edges
    let xedge2 = xedge + 1;
    let yedge2 = yedge + 1;

    //calculate lerp weights
    let weightx = x - xedge as f32;
    let weighty = y - yedge as f32;


    let g =  dot_grid_gradient(xedge, yedge, x, y, gradients);
    let g2 = dot_grid_gradient(xedge2, yedge, x, y, gradients);
    let noise = lerp(g, g2, weightx); // todo: actually calc weights

    let g =  dot_grid_gradient(xedge, yedge2, x, y, gradients);
    let g2 = dot_grid_gradient(xedge2, yedge2, x, y, gradients);
    let noise2 = lerp(g, g2, weightx); // todo: actually calc weights

    let value = lerp(noise, noise2, weighty);
    return value;

}



pub struct TerrainGenSystem{
    complete: bool,
    gradients: [[[f32 ; 2];100];100], //todo: calculate how big this should actually be, reduce redundancy
}

impl TerrainGenSystem{
    pub fn new() -> Self{
        TerrainGenSystem{complete:false, gradients: gradient_gen()} // true if terrain has been genned
    }
}

impl<'s> System<'s> for TerrainGenSystem {
    type SystemData = (
        WriteStorage<'s, Tiles>,
        WriteStorage<'s, Layer1>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, TilePos>,
    );

    fn run(&mut self, (mut tiles, layer1, mut spriterenderers, tileposses): Self::SystemData) {
        if !self.complete{ 
            for (tile, spriterender, pos, _) in (&mut tiles, &mut spriterenderers, & tileposses, & layer1).join(){
                if perlin(pos.x as f32 / 10., pos.y as f32 / 10. , self.gradients) > 0.10 { // divide by num to zoom into noise map
                    spriterender.sprite_number = 2 as usize; // trees
                }
                if perlin(pos.x as f32 / 10., pos.y as f32 / 10. , self.gradients) > 0.18 { 
                    spriterender.sprite_number = 3 as usize; // mountains
                }
                if perlin(pos.x as f32 / 10., pos.y as f32 / 10. , self.gradients) < 0.01 { 
                    spriterender.sprite_number = 1 as usize; // water
                }

            }

        }


        self.complete = true;
    }


}