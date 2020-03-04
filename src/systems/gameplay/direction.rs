
use amethyst::{
    input::{VirtualKeyCode, InputHandler, StringBindings},
    ecs::{Join, Read, WriteStorage, System},
    core::transform::Transform,
};

use snake::{Segment,SegmentType,SegmentDirection};

pub struct HeadDirectionSystem;

impl<'s> System<'s> for HeadDirectionSystem {
    
    type SystemData = (
        WriteStorage<'s,Transform>,
        WriteStorage<'s, Segment>,
        Read<'s, InputHandler<StringBindings>,
    );
    
    fn run(&mut self, (mut _transforms,mut segments,input) : Self::SystemData) {
        { 
            let head = (&mut segments).join().find(|s| s.t == SegmentType::Head).unwrap();
            if let Some(key) = input.keys_that_are_down().last() {
                head.direction = match key {
                    VirtualKeyCode::W => SegmentDirection::Up,
                    VirtualKeyCode::A => SegmentDirection::Left,
                    VirtualKeyCode::S => SegmentDirection::Down,
                    VirtualKeyCode::D => SegmentDirection::Right,
                    _ => return,
                }
            };
        }
    }
}