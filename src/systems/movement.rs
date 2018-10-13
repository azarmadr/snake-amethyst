use amethyst::{State,GameData,StateData,GameDataBuilder};
use amethyst::shred::System;
use amethyst::core::timing::Time;
use amethyst::ecs::prelude::{Resources,Join,Read,World,WriteStorage,ReadStorage,Write};
use amethyst::core::transform::Transform;
use amethyst::input::InputHandler;
use amethyst::core::cgmath::{Vector3,Vector2 ,Point3,Angle,Rad};
use amethyst::renderer::VirtualKeyCode;


use snake::{HeadSegment,BodySegment};

pub struct SnakeMovementSystem {
    pub segments: Vec<BodySegment>,
}

impl<'s> System<'s> for SnakeMovementSystem {
    type SystemData = (
        WriteStorage<'s,Transform>,
        Read<'s, InputHandler<String,String>>,
        ReadStorage<'s, HeadSegment>
    );
    
    fn run(&mut self, (mut transforms,input, head_segs) : Self::SystemData) {
        if input.key_is_down(VirtualKeyCode::W) {
            println!("W is down.");
        } else if input.key_is_down(VirtualKeyCode::A) {

        } else if input.key_is_down(VirtualKeyCode::S) {

        } else {
            // D
        }
        
        if let Some((pos,head)) = (&mut transforms, &head_segs).join().next() {

        }

    }
}