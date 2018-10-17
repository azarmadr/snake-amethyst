use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,Read,WriteStorage,Write,ReadStorage,Entities};
use amethyst::core::transform::Transform;
use amethyst::core::cgmath::Vector3;


pub struct CollisionSystem;

use snake::{Segment,SegmentType,SegmentDirection};
use spawnables::Food;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s,Transform>,
        ReadStorage<'s, Segment>,
        WriteStorage<'s, Food>,
    );
    
    fn run(&mut self, (entities,transforms,segments,mut foods) : Self::SystemData) {
        let (head_pos,_) = (&transforms, &segments).join().find(|(_,s)| s.t == SegmentType::Head).unwrap();
        for (e,food_pos,food) in (&*entities, &transforms, &mut foods).join() {
            if food_pos == head_pos {
                food.0 = true;
            }
        }
    }
}