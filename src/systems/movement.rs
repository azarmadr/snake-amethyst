use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,Read,WriteStorage,Write};
use amethyst::core::transform::Transform;
use amethyst::input::InputHandler;
use amethyst::core::cgmath::Vector3;
use amethyst::renderer::VirtualKeyCode;


use snake::{Segment,SegmentType,SegmentDirection,Snake};

pub struct SnakeMovementSystem;

impl<'s> System<'s> for SnakeMovementSystem {
    type SystemData = (
        WriteStorage<'s,Transform>,
        WriteStorage<'s, Segment>,
        Read<'s, InputHandler<String,String>>,
    );
    
    fn run(&mut self, (mut transforms,mut segments,input) : Self::SystemData) {
        let (pos,head) = (&mut transforms, &mut segments).join().find(|(_,s)| s.t == SegmentType::Head).unwrap();

        let pressed_keys: Vec<VirtualKeyCode> = input.keys_that_are_down().collect();
        if let Some(key) = &pressed_keys[..].last() {
            head.direction = match key {
                VirtualKeyCode::W => SegmentDirection::Up,
                VirtualKeyCode::A => SegmentDirection::Left,
                VirtualKeyCode::S => SegmentDirection::Down,
                VirtualKeyCode::D => SegmentDirection::Right,
                _ => return,
            }
        };

        match head.direction {
            SegmentDirection::Up => {pos.translation += Vector3::new(0.0,8.0,0.0);},
            SegmentDirection::Left => {pos.translation += Vector3::new(-8.0,0.0,0.0);},
            SegmentDirection::Down => {pos.translation += Vector3::new(0.0,-8.0,0.0);},
            SegmentDirection::Right => {pos.translation += Vector3::new(8.0,0.0,0.0);},
            _ => (),
        }
    }
}