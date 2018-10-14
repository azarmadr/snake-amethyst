use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,Read,WriteStorage,Write,ReadExpect,Entities,Resources};
use amethyst::ecs::World;
use amethyst::core::transform::Transform;
use amethyst::input::InputHandler;
use amethyst::core::cgmath::Vector3;
use amethyst::renderer::ScreenDimensions;
use amethyst::core::timing::Stopwatch;

use std::time::{Duration,Instant};
use spawnables::Food;
use rand;
use rand::Rng;

pub struct SpawningSystem{
    pub respawn_time: Stopwatch,
}

impl<'s> System<'s> for SpawningSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>, 
        WriteStorage<'s, Food>,
        ReadExpect<'s, ScreenDimensions>,
    );
    fn setup(&mut self, _res: &mut Resources) {
        self.respawn_time = Stopwatch::Started(Duration::from_millis(0), Instant::now());
    }
    fn run(&mut self, (entities, mut transforms, mut foods,dimns) : Self::SystemData) {
        if self.respawn_time.elapsed() > Duration::from_millis(2000) {
            let mut transform = Transform::default();

            let (width,height) = {
                (dimns.width(), dimns.height())
            };

            let x = rand::thread_rng().gen_range(0,width as u32) as f32;
            let y = rand::thread_rng().gen_range(0,height as u32) as f32;
            transform.translation = Vector3::new(x,y,0.0);

           entities.build_entity()
                    .with(transform, &mut transforms)
                    .with(Food::default(), &mut foods)
                    .build();
            self.respawn_time.restart();
        }

    }
}