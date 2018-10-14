use amethyst::prelude::*;

use utilities::{
    decompile_as_sprites,
    initialise_camera,
};
use snake;
use spawnables::Food;

pub struct SnakeGame;

impl<'a, 'b> State<GameData<'a, 'b>> for SnakeGame {
    fn on_start(&mut self, data: StateData<GameData>) {
        let mut world = data.world;
        world.register::<Food>();
        initialise_camera(&mut world);
        
        let sheet_handle = decompile_as_sprites(&mut world, "SnakeSprite.png", (16.0,32.0), (8.0,8.0), 0);
        snake::initialise_snake(&mut world, sheet_handle.clone());
    }
    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>>{
        data.data.update(&data.world);
        Trans::None
    }
}