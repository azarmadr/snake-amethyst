use amethyst::{
    shrev::{EventChannel, ReaderId},
    core::{
        math::{UnitQuaternion, Vector3},
        Time, Transform, SystemDesc,
    },
    derive::SystemDesc,
    ecs::prelude::{
        World,
        Write, Read, WriteExpect,
        Entity, Join, ReadStorage, System, SystemData, WriteStorage,
    },
    renderer::{camera::Camera, light::Light},
    ui::{UiFinder, UiText, UiEvent, UiTransform, UiEventType},
    utils::fps_counter::FpsCounter,
};

use crate::utilities::UiGenerator;

pub struct UiEventHandlerSystem {
    reader_id: ReaderId<UiEvent>,
}

impl UiEventHandlerSystem {
    pub fn new(reader_id: ReaderId<UiEvent>) -> Self {
        Self { reader_id }
    }
}

impl<'a> System<'a> for UiEventHandlerSystem {
    type SystemData = Write<'a, EventChannel<UiEvent>>;

    fn run(&mut self, (events): Self::SystemData) {
        for event in events.read(&mut self.reader_id) {
            match event {
                UiEvent {
                    event_type: UiEventType::Click,
                    target,
                } => { 

                     },
                UiEvent {
                    event_type: UiEventType::HoverStart,
                    target,
                } => { 
                    
                     },
                UiEvent {
                    event_type: UiEventType::HoverStart,
                    target,
                } => { 

                    
                     },
                _ => {},
            }
        }
    }
}

pub struct UiEventHandlerDesc;

impl<'a, 'b> SystemDesc<'a, 'b, UiEventHandlerSystem> for UiEventHandlerDesc {
    fn build(self, world: &mut World) -> UiEventHandlerSystem {
        <UiEventHandlerSystem as System<'_>>::SystemData::setup(world);

        let reader = world.fetch_mut::<EventChannel<UiEvent>>().register_reader();

        UiEventHandlerSystem::new(reader)
    }
}