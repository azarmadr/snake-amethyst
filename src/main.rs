extern crate amethyst;
extern crate rand;

//mod systems;
mod cgd;
mod states;
mod resources;
mod components;
mod utilities;

use amethyst::{
    core::{transform::TransformBundle, frame_limiter::FrameRateLimitStrategy},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        RenderFlat2D,
        plugins::{RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle, NoCustomUi},
    utils,
};

use states::LoadingState;




fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root_dir = utils::application_root_dir()?;
    let resources_dir = root_dir.join("resources");
    let assets_dir = root_dir.join("assets");

    let display_config = resources_dir.join("display_config.ron");
    let input_config = resources_dir.join("input_config.ron");

    let input_bundle = InputBundle::<StringBindings>::new();
    let input_bundle = match input_bundle.with_bindings_from_file(input_config) {
        Ok(ip) => {
            ip
        },
        Err(_) => {
            InputBundle::<StringBindings>::new()
        }
    };

    let game_data = cgd::GameExecBuilder::default()
        .with_base_bundle(TransformBundle::new())
        .with_base_bundle(input_bundle)
        .with_base_bundle(UiBundle::<StringBindings, NoCustomUi, u32, u32>::new())
        .with_base_bundle( 
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
        );

    Application::build(assets_dir, LoadingState::default())?
        .build(game_data)?
        .run();
    Ok(())
}