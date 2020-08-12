use amethyst::{
    core::{
        transform::Transform,
        Time
    },
    derive::SystemDesc,
    renderer::{Camera},
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings}, 
};

#[derive(SystemDesc)]
pub struct CameraSystem{
    multiplier : i32,
}

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );


    fn run(&mut self, (mut transforms, mut cameras, input, time): Self::SystemData) {
        for (_camera, transform) in (&mut cameras, &mut transforms).join() {
            if let Some(mv_amount) = input.axis_value("updown") {
                let scaled_amount = -200. * mv_amount * time.delta_seconds();
                let camera_y = transform.translation().y;
                transform.set_translation_y(camera_y + scaled_amount);
            } 
            if let Some(mv_amount) = input.axis_value("leftright") {
                let scaled_amount = -200. * mv_amount * time.delta_seconds();
                let camera_x = transform.translation().x ;
                transform.set_translation_x(camera_x + scaled_amount);
            }
        }
        self.multiplier = 1;
        if input.action_is_down("speed_up"){
            self.multiplier = 2
        }
    }
}