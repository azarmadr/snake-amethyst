use amethyst::{
    window::ScreenDimensions,
    ecs::{Join, WriteStorage, ReadStorage, Entities, ReadExpect, System},
    core::transform::Transform,
};

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
        // Acquiring the head segment.
        let (head_trans,_) = (&transforms, &segments).join().find(|(_,s)| s.t == SegmentType::Head).unwrap();

        // Checking if the head collides with any other body segments.
        if let Some(_) = (&transforms, &segments).join()
                                                 .filter(|(_,s)| s.t == SegmentType::Body)
                                                 .find(|(t,_)| t == &head_trans) {
            panic!("Snake eating itself.");
        } 
        // Checking if the head has collided with the food.
        // Then we set the field to true and in our `removal.rs` we remove all collided food types. 
        for (_e,food_pos,food) in (&*entities, &transforms, &mut foods).join() {
            if food_pos == head_trans {
                food.0 = true;
            }
        }
        // Checking if the head collides with the border / window dimensions.
        let pos = head_trans.translation;
        if pos.x < 0.0 || pos.y < 0.0 || pos.x > dimensions.width() || pos.y > dimensions.height() {
            panic!("Snake went out of bounds.");
        } 
    }
}