pub mod collision;
pub mod movement;

use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::ecs::prelude::DispatcherBuilder;

use self::movement::SnakeMovementSystem;

pub struct SnakeSystemBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SnakeSystemBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(SnakeMovementSystem { segments: vec![]}, "player_movement", &[]);
        Ok(())
    }
}