use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,ReadStorage,Entities};

pub struct PowerUpSystem;

use snake::{Segment};
use spawnables::Food;

impl<'s> System<'s> for PowerUpSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Food>,
        ReadStorage<'s, Segment>,
    );
    
    fn run(&mut self, (entities,mut foods,_segments) : Self::SystemData) {
    }
}