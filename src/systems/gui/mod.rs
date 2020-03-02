use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::ecs::prelude::DispatcherBuilder;

pub struct MenuSystemBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MenuSystemBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        Ok(())
    }
}