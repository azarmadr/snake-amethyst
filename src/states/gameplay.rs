use amethyst::{
    input::{InputEvent,VirtualKeyCode},
    State, StateData, Trans, StateEvent,
};

use crate::cgd::GameExecData;

pub struct GameplayState;

impl<'a, 'b> State<GameExecData<'a, 'b>, StateEvent> for GameplayState {
    
    fn on_start(&mut self,data: StateData<GameExecData>) {
        println!("Statring the Gameplay.");
        let world = data.world;
    }

    fn update(&mut self,data: StateData<GameExecData>) -> Trans<GameExecData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);
        Trans::None
    }

    fn handle_event(&mut self, data: StateData<GameExecData>, event: StateEvent,) -> Trans<GameExecData<'a, 'b>, StateEvent> {
        let world = data.world;

        match event {
            StateEvent::Input(
                InputEvent::KeyPressed {
                    key_code: VirtualKeyCode::Escape,
                    scancode: _,
                }
            ) => {
                return Trans::Pop;
            },
            _ => Trans::None,
        }
    }
}