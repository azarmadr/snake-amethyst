use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,ReadStorage,Entities};

pub struct RemovalSystem;

use snake::{Segment};
use spawnables::Food;

impl<'s> System<'s> for RemovalSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Food>,
        ReadStorage<'s, Segment>,
    );
    
    fn run(&mut self, (entities,mut foods,_segments) : Self::SystemData) {
        if let Some((e,_)) = (&*entities,&mut foods).join().filter(|(_,f)| f.0 == true).next() {
            let _ = entities.delete(e);
        }
    }
}