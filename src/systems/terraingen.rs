use crate::game::{Layer1, TilePos, TileType, Tiles};

use amethyst::{
    ecs::prelude::{Join, System, WriteStorage},
    renderer::SpriteRender
};

use rand::prelude::*;

// generates the 2d unit length gradient vectors
// Todo: implement seeded random?
fn gradient_gen() -> [[[f32 ; 2];100];100]{ // todo: convert this to a map function?
    let mut gradient_array = [[[0. ; 2];100];100]; 
    for x in 0..100{
        for y in 0..100{
            let mut rng = rand::thread_rng();
            // todo: this might be a bit funky (v small x values) look into it a bit if terrain gets funky
            let gx = rng.gen::<f32>(); // here a random x value is genned, to get the y value of the unit vector
            let gy = (1.-(gx*gx)).sqrt();   // we perform the 1 = sqrt(x^2+y^2) rearranged
            gradient_array[x][y][0] = gx;
            gradient_array[x][y][1] = gy;
        }
    }
    return gradient_array;
}

// linear interpolates between v0 and v1 using weight w (w should be somewhere between 0.0 and 1.0)
// weight describes where to "estimate the value on the interpolated line"
fn lerp(v0: f32, v1: f32, w: f32) -> f32{ // http://paulbourke.net/miscellaneous/interpolation/ to understand better.
    return w*v1 + (1.0-w)*v0
}

// returns the dot product of the distance and gradient vectors
fn dotp_grid(ix: i32, iy: i32, x : f32, y : f32, gradients : [[[f32 ; 2];100];100]) -> f32{ // iy and ix are the vectors at each node for each tile (4 nodes per tile)

    // calculate the distance between the point and the nodes
    let dx = x - ix as f32;
    // For each gradient, we calculate the offset vector from the corner to the
    // candidate point. We take the dot product between each pair of gradient
    // vector and offset vector. This value will be zero if the candidate point
    // is exactly at a grid corner.
    let dy = y - iy as f32;

    // this bit calculates the dot product:
    // https://en.wikipedia.org/wiki/Dot_product#Algebraic_definition. The dot
    // product "takes 2 equal length sequences of numbers and returns a single
    // number" basically converts a vector into a scalar. We ended up doing dot
    // product in lesson which is pretty cool.
    return dx*gradients[ix as usize][iy as usize][0] + dy* gradients[ix as usize][iy as usize][1]

}

fn perlin(x : f32, y: f32, gradients : [[[f32 ; 2];100];100]) -> f32{

    let xedge = x.floor() as i32;
    let yedge = y.floor() as i32;

    // calculate lerp weights
    // these are the "offset vector from the corner to the target point"
    let weightx = x - xedge as f32;
    let weighty = y - yedge as f32;

    //Take the dot product between each pair of gradient vectors
    let mut g =  dotp_grid(xedge, yedge, x, y, gradients);
    let mut g2 = dotp_grid(xedge + 1, yedge, x, y, gradients);
    let noise = lerp(g, g2, weightx); // Done: actually calc weights
    g =  dotp_grid(xedge, yedge + 1, x, y, gradients);
    g2 = dotp_grid(xedge + 1, yedge + 1, x, y, gradients);
    let noise2 = lerp(g, g2, weightx);
    // ^ I stumbled through this bit, but it seems to be how almost every other
    // implementation handles it (or at least very similar) and it seems to work.

    // Interpolate bettween all the dot products to smooth the noise
    let value = lerp(noise, noise2, weighty);
    return value;

}



pub struct TerrainGenSystem{
    complete: bool,
    gradients: [[[f32 ; 2];100];100], //todo: calculate how big this should actually be, reduce redundancy. These values work for now.
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
                if perlin(pos.x as f32 / 10., pos.y as f32 / 10. , self.gradients) > 0.1 { // divide by num to zoom into noise map
                    spriterender.sprite_number = TileType::Forest as usize;
                    tile.tile_type = Some(TileType::Forest);
                }
                if perlin(pos.x as f32 / 10., pos.y as f32 / 10. , self.gradients) > 0.18 { 
                    spriterender.sprite_number = TileType::Mountains as usize; // mountains
                    tile.tile_type = Some(TileType::Mountains);
                }
                if perlin(pos.x as f32 / 10., pos.y as f32 / 10. , self.gradients) < 0.025 { 
                    spriterender.sprite_number = TileType::Sea as usize; // sea
                    tile.tile_type = Some(TileType::Sea);
                }

            }

        }


        self.complete = true;
    }


}
