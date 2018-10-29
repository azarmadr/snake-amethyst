use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,ReadStorage,Entities,ReadExpect};
use amethyst::core::transform::Transform;
use amethyst::renderer::ScreenDimensions;

pub struct CollisionSystem;

use snake::{Segment,SegmentType};
use spawnables::Food;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s,Transform>,
        ReadStorage<'s, Segment>,
        WriteStorage<'s, Food>,
        ReadExpect<'s, ScreenDimensions>,
    );
    
    fn run(&mut self, (entities,transforms,segments,mut foods,dimensions) : Self::SystemData) {
        let (head_pos,_) = (&transforms, &segments).join().find(|(_,s)| s.t == SegmentType::Head).unwrap();
        // Checking if the head has collided with the food.
        // Then we set the field to true and in our `removal.rs` we remove all collided food types. 
        for (_e,food_pos,food) in (&*entities, &transforms, &mut foods).join() {
            if food_pos == head_pos {
                food.0 = true;
            }
        }
        
        let pos = head_pos.translation;
        if pos.x < 0.0 || pos.y < 0.0 || pos.x > dimensions.width() || pos.y > dimensions.height() {
            panic!("Snake went out of bounds.");
        } 
    }
}