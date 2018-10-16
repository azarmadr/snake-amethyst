use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,Read,WriteStorage,Write,ReadExpect,Entities,WriteExpect,Resources};
use amethyst::ecs::World;
use amethyst::core::transform::{GlobalTransform,Transform};
use amethyst::input::InputHandler;
use amethyst::core::cgmath::Vector3;
use amethyst::renderer::ScreenDimensions;
use amethyst::core::timing::Stopwatch;
use amethyst::renderer::SpriteRender;


use std::time::{Duration,Instant};
use spawnables::Food;
use rand;
use rand::Rng;

use game::Backpack;

pub struct SpawningSystem{
    pub respawn_time: Stopwatch,
}

impl<'s> System<'s> for SpawningSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>, 
        WriteStorage<'s, Food>,
        ReadExpect<'s, ScreenDimensions>,
        ReadExpect<'s, Backpack>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, GlobalTransform>,
    );
    fn setup(&mut self, _res: &mut Resources) {
        self.respawn_time = Stopwatch::Started(Duration::from_millis(0), Instant::now());
    }
    fn run(&mut self, (entities, mut transforms, mut foods,dimns,backpack,mut sprites,mut gtransforms) : Self::SystemData) {
        if self.respawn_time.elapsed() > Duration::from_millis(3000) {
            let mut transform = Transform::default();
            
            if let None = transforms.join().find(|t| t.translation == transform.translation) {
                transform.translation = generate_random_pos(dimns.width(),dimns.height());
                println!("Food at: {:?}", transform.translation);
            } else {
                return;
            }

            let sprite_sheet = if let Some(ref sheet) = backpack.food_sheet {
                sheet.clone()
            } else {
                return;
            };

            let food_color_id = rand::thread_rng().gen_range(0,4);

            let food_sprite = SpriteRender {
                sprite_sheet: sprite_sheet,
                sprite_number: food_color_id,
                flip_horizontal: false,
                flip_vertical: false,
            };

           entities.build_entity()
                    .with(transform, &mut transforms)
                    .with(GlobalTransform::default(),&mut gtransforms)
                    .with(food_sprite, &mut sprites)
                    .with(Food::default(), &mut foods)
                    .build();
            self.respawn_time.restart();
        }

    }
}

pub fn generate_random_pos(width: f32,height: f32) -> Vector3<f32> {
    let (mut x,mut y) = (1, 1);
    while x % 8 != 0 && y % 8 != 0 {
        x = rand::thread_rng().gen_range(0,width as u32);
        y = rand::thread_rng().gen_range(0,height as u32);
    }
    Vector3::new(x as f32,y as f32,0.0)
}