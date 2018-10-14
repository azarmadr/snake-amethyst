pub mod collision;
pub mod movement;
pub mod spawning;

use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::ecs::prelude::DispatcherBuilder;
use amethyst::core::timing::Stopwatch;

use self::movement::SnakeMovementSystem;
use self::spawning::SpawningSystem;

pub struct SnakeSystemBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SnakeSystemBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(SnakeMovementSystem, "head_movement", &[]);
        builder.add(SpawningSystem { respawn_time: Stopwatch::new() }, "spawining_system", &[]);
        Ok(())
    }
}