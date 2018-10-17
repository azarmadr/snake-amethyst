use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,Read,WriteStorage,Write};
use amethyst::core::transform::Transform;
use amethyst::input::InputHandler;
use amethyst::core::cgmath::Vector3;
use amethyst::renderer::VirtualKeyCode;


use snake::{Segment,SegmentType,SegmentDirection};

pub struct SnakeMovementSystem;

impl<'s> System<'s> for SnakeMovementSystem {
    type SystemData = (
        WriteStorage<'s,Transform>,
        WriteStorage<'s, Segment>,
    );
    
    fn run(&mut self, (mut transforms,mut segments) : Self::SystemData) {
        for (transform, segment) in (&mut transforms,&mut segments).join() {
            transform.translation += match segment.direction {
                SegmentDirection::Up => Vector3::new(0.0,8.0,0.0),
                SegmentDirection::Left => Vector3::new(-8.0,0.0,0.0),
                SegmentDirection::Down => Vector3::new(0.0,-8.0,0.0),
                SegmentDirection::Right => Vector3::new(8.0,0.0,0.0),
                _ => Vector3::new(0.0,0.0,0.0),
            };
        }
    }
}