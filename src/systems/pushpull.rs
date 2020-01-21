use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{System, SystemData, Join, WriteStorage, ReadStorage, Read, World},
    input::{InputHandler, StringBindings},
};
use crate::pong::{Ball, PUSHPULL_FORCE};

#[derive(SystemDesc)]
pub struct PushPullSystem;

impl<'s> System<'s> for PushPullSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut balls, entities, input): Self::SystemData) {
        //is there a way to get an iterator without joining?
        for (ball) in (&mut balls).join() {
            if let Some(pushpull_amount) = input.axis_value("left_pushpull") {
                ball.velocity[0] += PUSHPULL_FORCE * pushpull_amount;
            }

            if let Some(pushpull_amount) = input.axis_value("right_pushpull") {
                ball.velocity[0] += PUSHPULL_FORCE * pushpull_amount;
            }
        }
    }
}