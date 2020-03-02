use amethyst::prelude::*;
use amethyst::ecs::prelude::{Join,Entities,WriteStorage,ReadStorage};
use amethyst::renderer::{Hidden,VirtualKeyCode};
use amethyst::input;
use amethyst::core::transform::Transform;
use amethyst::ui::UiTransform;
use utilities::{
    decompile_as_sprites,
    initialise_camera,
    Backpack,
    initialize_ui,
};
use custom_game_data::CustomGameData;
use snake::initialise_snake;

pub struct SnakeGame;

impl<'a, 'b> State<CustomGameData<'a,'b>,StateEvent> for SnakeGame {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        let mut world = data.world;


        let snake_sheet_handle = decompile_as_sprites(&mut world, "SnakeSprite.png", (16.0,32.0), (8.0,8.0), 0);
        let food_sheet_handle = decompile_as_sprites(&mut world, "FoodSprite.png", (16.0,16.0), (8.0,8.0), 1);
        
        initialise_camera(&mut world);
        initialise_snake(&mut world, snake_sheet_handle.clone());

        world.add_resource(Backpack::new(snake_sheet_handle,food_sheet_handle));
    }
    fn handle_event(&mut self, _data: StateData<CustomGameData>, event: StateEvent) ->  Trans<CustomGameData<'a,'b>,StateEvent> {
        if let StateEvent::Window(e) = event {
            if input::is_key_down(&e, VirtualKeyCode::Escape ) {
                return Trans::Push(Box::new(SnakeMenu));
            } 
        }
        Trans::None
    }

    fn fixed_update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a,'b>,StateEvent>{
        data.data.update(&data.world,true);
        Trans::None
    }
    fn on_pause(&mut self, data: StateData<CustomGameData>) {
        data.world.exec(|(entities,tranforms,mut hidden_storage): (Entities,ReadStorage<Transform>,WriteStorage<Hidden>)| {
            for (e,_) in (&*entities,&tranforms).join() {
                let _ = hidden_storage.insert(e, Hidden);
            }
        });
    }
    fn on_resume(&mut self, data: StateData<CustomGameData>) {
        data.world.exec(|(entities,tranforms,mut hidden_storage): (Entities,ReadStorage<Transform>,WriteStorage<Hidden>)| {
            for (e,_) in (&*entities,&tranforms).join() {
                hidden_storage.remove(e);
            }
        });
    }
}

pub struct SnakeMenu;

impl<'a, 'b> State<CustomGameData<'a,'b>,StateEvent> for SnakeMenu {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        let mut world = data.world;

        initialize_ui(&mut world);
    }

    fn handle_event(&mut self, _data: StateData<CustomGameData>, event: StateEvent) ->  Trans<CustomGameData<'a,'b>,StateEvent> {
        if let StateEvent::Window(e) = event {
            if input::is_key_down(&e, VirtualKeyCode::Escape ) {
                return Trans::Pop;
            } 
        }
        Trans::None
    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a,'b>,StateEvent>{
        data.data.update(&data.world,false);
        Trans::None
    }
    fn on_stop(&mut self, data: StateData<CustomGameData>) {
        data.world.exec(|(entities,mut ui_transform_storage): (Entities,WriteStorage<UiTransform>)| {
            for (e,_) in (&*entities,&mut ui_transform_storage).join() {
                let _ = entities.delete(e);
            }
        });
    }
}