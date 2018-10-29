use amethyst::prelude::*;
use amethyst::renderer::SpriteSheetHandle;
use utilities::{
    decompile_as_sprites,
    initialise_camera,
};
use snake::{Segment,initialise_snake};
use spawnables::Food;

pub struct SnakeGame;

impl<'a, 'b> State<GameData<'a, 'b>,StateEvent> for SnakeGame {
    fn on_start(&mut self, data: StateData<GameData>) {
        let mut world = data.world;

        world.register::<Food>();
        world.register::<Segment>();

        let snake_sheet_handle = decompile_as_sprites(&mut world, "SnakeSprite.png", (16.0,32.0), (8.0,8.0), 0);
        let food_sheet_handle = decompile_as_sprites(&mut world, "FoodSprite.png", (16.0,16.0), (8.0,8.0), 1);
        
        initialise_camera(&mut world);
        initialise_snake(&mut world, snake_sheet_handle.clone());

        world.add_resource(Backpack::new(snake_sheet_handle,food_sheet_handle));
    }

    fn update(&mut self, data: StateData<GameData<'a,'b>>) -> Trans<GameData<'a,'b>,StateEvent>{
        data.data.update(&data.world);
        Trans::None
    }
}

pub struct Backpack {
    pub snake_sheet: Option<SpriteSheetHandle>,
    pub food_sheet: Option<SpriteSheetHandle>,
}
impl Backpack {
    pub fn new(sheet: SpriteSheetHandle,food_sheet: SpriteSheetHandle) -> Self {
        Backpack {
            snake_sheet: Some(sheet),
            food_sheet: Some(food_sheet),
        }
    }
}
impl Default for Backpack {
    fn default() -> Self {
        Backpack {
            snake_sheet: None,
            food_sheet: None,
        }
    }
}