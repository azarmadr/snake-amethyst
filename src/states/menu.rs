use amethyst::{
    ecs::{WorldExt, Entity},
    State, StateData, Trans, StateEvent,
    core::Hidden,
    ui::{UiText,UiEvent, UiEventType},
};
use crate::{
    cgd::GameExecData,
    utilities::{UiGenerator,UiEffect},
    resources::UiResources,
};

use std::collections::HashMap;


pub struct MenuState {
    ui_res: UiResources,
    widgets: HashMap<Entity, UiEffect>,
}

impl MenuState {

    pub fn new(ui_res: UiResources) -> Self {
        let widgets = HashMap::new();

        Self {
            ui_res,
            widgets,
        }
    }
}

impl<'a, 'b> State<GameExecData<'a, 'b>, StateEvent> for MenuState {
    fn on_start(&mut self,data: StateData<GameExecData>) {
        println!("Switched to `MenuState`");

        let world = data.world;
        
        crate::utilities::initialize_camera(world);

        let buttons = vec![
            ("NEW GAME", UiEffect::NewGame, false),
            ("OPTIONS",  UiEffect::Options, false),
            ("QUIT",     UiEffect::Quit, false),
        ];

        self.widgets = UiGenerator::generate_buttons(world,self.ui_res.clone(), buttons);
    }
    fn handle_event(&mut self, data: StateData<GameExecData>, event: StateEvent,) -> Trans<GameExecData<'a, 'b>, StateEvent> {
        let world = data.world;

        match event {
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                let mut text_storage = world.write_storage::<UiText>();
                if let Some(txt) = text_storage.get_mut(target) {
                    UiGenerator::unset_hover_color(txt);
                }
                
                if let Some(ui_effect) = self.widgets.get(&target) {
                    return UiEffect::as_trans(*ui_effect, self.ui_res.clone());
                } 
                Trans::None
            },
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::HoverStart,
                target,
            }) => {
                let mut text_storage = world.write_storage::<UiText>();
                if let Some(txt) = text_storage.get_mut(target) {
                    UiGenerator::set_hover_color(txt);
                }
                Trans::None
            },
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::HoverStop,
                target,
            }) => {
                let mut text_storage = world.write_storage::<UiText>();
                if let Some(txt) = text_storage.get_mut(target) {
                    UiGenerator::unset_hover_color(txt);
                }
                Trans::None
            },
            _ => Trans::None,
        }
    }

    fn update(&mut self,data: StateData<GameExecData>) -> Trans<GameExecData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);
        Trans::None
    }

    fn on_pause(&mut self,data: StateData<GameExecData>) {
        let world = data.world;
        
        let mut hidden_storage = world.write_storage::<Hidden>();
        
        for e in self.widgets.keys() {
            let _ = hidden_storage.insert(*e, Hidden);
        }
    }
    
    fn on_resume(&mut self,data: StateData<GameExecData>) {
        let world = data.world;
        
        let mut hidden_storage = world.write_storage::<Hidden>();
        for e in self.widgets.keys() {
            let _ = hidden_storage.remove(*e);
        }

    }
}