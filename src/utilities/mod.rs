use amethyst::{
    window::ScreenDimensions,
    prelude::{World,WorldExt},
    ecs::prelude::{Builder},
    core::transform::components::Transform,
    renderer::{Camera,camera::Projection},
};
 

mod ui;

pub use self::ui::UiGenerator;


pub fn acquire_screen_dimensions(world: &mut World) -> (f32,f32) {
    let dims = world.read_resource::<ScreenDimensions>();
    (dims.width(), dims.height())
}

pub fn initialize_camera(world: &mut World) {
    let (width, height) = acquire_screen_dimensions(world);
    let mut camera_transform = Transform::default();
    camera_transform.set_translation_xyz( 100f32, height / 2f32, 100f32);
    world.create_entity()
        .with(Camera::from(Projection::perspective(width / height,  90f32, 100f32, 2000f32)))
        .with(camera_transform)
        .build();
}