use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,Read,WriteStorage};
use amethyst::core::transform::Transform;
use amethyst::input::InputHandler;
use amethyst::core::cgmath::Vector3;
use amethyst::renderer::VirtualKeyCode;


use snake::{Segment,SegmentType,SegmentDirection};

pub struct HeadDirectionSystem;

impl<'s> System<'s> for HeadDirectionSystem {
    type SystemData = (
        WriteStorage<'s,Transform>,
        WriteStorage<'s, Segment>,
        Read<'s, InputHandler<String,String>>,
    );
    
    fn run(&mut self, (mut transforms,mut segments,input) : Self::SystemData) {
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
        { 
            let mut elements: Vec<_> = (&mut segments).join().collect();
            &elements[..].sort_by_key(|e| e.id ); 
            for idx in 0..elements.len() {
                if idx == elements.len() - 1 { 
                    return;
                } 
                if elements.len() == 1 {
                    continue;
                }
                elements[idx+1].direction = elements[idx].direction;
            }
        }

    }
}