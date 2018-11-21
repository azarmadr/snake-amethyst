pub mod collision;
pub mod movement;
pub mod spawning;
pub mod direction;
pub mod removal;

use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::ecs::prelude::DispatcherBuilder;


use self::movement::SnakeMovementSystem;
use self::spawning::SpawningSystem;
use self::collision::CollisionSystem;
use self::direction::HeadDirectionSystem;
use self::removal::RemovalSystem;

pub struct SnakeSystemBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SnakeSystemBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(HeadDirectionSystem, "head_movement", &[]);
        builder.add(CollisionSystem, "collision_system", &[]);
        builder.add(SpawningSystem, "spawning_system", &["collision_system"]);
        builder.add(SnakeMovementSystem::default(), "snake_movement",  &["head_movement"]); 
        builder.add(RemovalSystem, "removal_system", &["spawning_system"]);
        Ok(())
    }
}