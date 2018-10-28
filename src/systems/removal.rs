use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,ReadStorage,Entities};
use amethyst::core::transform::Transform;


pub struct RemovalSystem;

use snake::{Segment,SegmentType,SegmentDirection};
use spawnables::Food;

impl<'s> System<'s> for RemovalSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Food>,
        ReadStorage<'s, Segment>,
    );
    
    fn run(&mut self, (entities,mut foods,segments) : Self::SystemData) {
        for (e,f) in (&*entities,&mut foods).join().filter(|(_,f)| f.0 == true) {
            let _ = entities.delete(e);
        }
    }
}