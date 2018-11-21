use amethyst::prelude::*;
use amethyst::renderer::SpriteSheetHandle;
use utilities::{
    decompile_as_sprites,
    initialise_camera,
    Backpack,
};
use custom_game_data::CustomGameData;

use snake::{initialise_snake};
use spawnables::Food;

pub struct SnakeGame{
    is_menu_open: bool
}
impl Default for SnakeGame {
    fn default() -> Self {
        Self {
            is_menu_open: true,
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a,'b>,StateEvent> for SnakeGame {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        let mut world = data.world;

        world.register::<Food>();

        let snake_sheet_handle = decompile_as_sprites(&mut world, "SnakeSprite.png", (16.0,32.0), (8.0,8.0), 0);
        let food_sheet_handle = decompile_as_sprites(&mut world, "FoodSprite.png", (16.0,16.0), (8.0,8.0), 1);
        
        initialise_camera(&mut world);
        initialise_snake(&mut world, snake_sheet_handle.clone());

        world.add_resource(Backpack::new(snake_sheet_handle,food_sheet_handle));
    }

    fn fixed_update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a,'b>,StateEvent>{
        data.data.update(&data.world,true);
        Trans::None
    }
}

struct SnakeMenu;

impl<'a, 'b> State<CustomGameData<'a,'b>,StateEvent> for SnakeMenu {
    fn on_start(&mut self, data: StateData<CustomGameData>) {

    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a,'b>,StateEvent>{
        data.data.update(&data.world,true);
        Trans::None
    }
}