use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,ReadExpect,Entities,WriteExpect,Resources};
use amethyst::core::transform::{GlobalTransform,Transform};
use amethyst::core::cgmath::Vector3;
use amethyst::renderer::ScreenDimensions;
use amethyst::core::timing::Stopwatch;
use amethyst::renderer::SpriteRender;


use std::time::{Duration,Instant};
use spawnables::Food;
use snake::{Segment,SegmentType,SegmentDirection,Snake};
use rand;
use rand::Rng;

use game::Backpack;

pub struct SpawningSystem{
    pub respawn_time: Stopwatch,
}
impl Default for SpawningSystem {
    fn default() -> Self {
        Self {
            respawn_time: Stopwatch::new(),
        }
    }
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
        WriteStorage<'s, Segment>,
        WriteExpect<'s, Snake>,
    );
    fn setup(&mut self, _res: &mut Resources) {
        self.respawn_time = Stopwatch::Started(Duration::from_millis(0), Instant::now());
    }
    fn run(&mut self, (entities, mut transforms, mut foods,dimns,backpack,mut sprites,mut gtransforms,mut segments,mut snake) : Self::SystemData) {
        for (e,food) in (&*entities,&mut foods).join() {
            if food.0 {
                snake.score += 1;

                let sprite_sheet = if let Some(ref sheet) = backpack.snake_sheet {
                        sheet.clone()
                    } else {
                        return;
                };

                let (mut pos,seg_dir,sprite_id) = { 
                    let (pos,seg,sprite) = (&transforms, &segments,&sprites).join().last().unwrap();
                    (pos.translation, seg.direction,sprite.sprite_number)
                };

                let snake_sprite = SpriteRender {
                    sprite_sheet: sprite_sheet,
                    sprite_number: sprite_id,
                    flip_horizontal: false,
                    flip_vertical: false,
                };

                let mut transform = Transform::default();
                pos += match seg_dir {
                    SegmentDirection::Up => Vector3::new(0.0,-8.0,0.0),
                    SegmentDirection::Left => Vector3::new(8.0,0.0,0.0),
                    SegmentDirection::Down => Vector3::new(0.0,8.0,0.0),
                    SegmentDirection::Right => Vector3::new(-8.0,0.0,0.0),
                    _ => Vector3::new(0.0,0.0,0.0),
                };
                
                transform.translation = pos;

                entities.build_entity()
                        .with(snake_sprite, &mut sprites)
                        .with(GlobalTransform::default(),&mut gtransforms)
                        .with(transform,&mut transforms)
                        .with(Segment::body(seg_dir, snake.score), &mut segments)
                        .build();
            }
        }
        
        if self.respawn_time.elapsed() > Duration::from_millis(3000) {
            let mut transform = Transform::default();
            
            if let None = transforms.join().find(|t| t.translation == transform.translation) {
                transform.translation = generate_random_pos(dimns.width(),dimns.height());
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
    let x = (rand::thread_rng().gen_range(0,width as u32) as f32 / 8.0).floor() * 8.0;
    let y = (rand::thread_rng().gen_range(0,height as u32) as f32 / 8.0).floor() * 8.0;
    Vector3::new(x,y,0.0)
}