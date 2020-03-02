use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,ReadExpect,Entities,WriteExpect};
use amethyst::core::transform::{GlobalTransform,Transform};
use amethyst::core::cgmath::Vector3;
use amethyst::renderer::ScreenDimensions;
use amethyst::renderer::SpriteRender;


use spawnables::Food;
use snake::{Segment,SegmentDirection,Snake};
use rand;
use rand::Rng;

use utilities::Backpack;

pub struct SpawningSystem;

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
    fn run(&mut self, (entities, mut transforms, mut foods,dimns,backpack,mut sprites,mut gtransforms,mut segments,mut snake) : Self::SystemData) {
        if let Some(_) = (&foods).join().filter(|f| f.0).next() {
            snake.score += 1;
            snake.food_available = false;

            let sprite_sheet = if let Some(ref sheet) = backpack.snake_sheet {
                    sheet.clone()
                } else {
                    return;
            };

            let (mut pos,seg_dir,sprite_id) = { 
                let mut custom_vec: Vec<_> = (&transforms, &segments,&sprites).join().collect();
                custom_vec.sort_by_key(|(_,seg,_)| seg.id);
                let (pos,seg,sprite) = custom_vec.pop().unwrap();
                (pos.translation, seg.direction,sprite.sprite_number)
            };

            let snake_sprite = SpriteRender {
                sprite_sheet: sprite_sheet,
                sprite_number: sprite_id,
                flip_horizontal: false,
                flip_vertical: false,
            };

            pos += match seg_dir {
                SegmentDirection::Up => Vector3::new(0.0,-8.0,0.0),
                SegmentDirection::Left => Vector3::new(8.0,0.0,0.0),
                SegmentDirection::Down => Vector3::new(0.0,8.0,0.0),
                SegmentDirection::Right => Vector3::new(-8.0,0.0,0.0),
                _ => Vector3::new(0.0,0.0,0.0),
            };
            
            entities.build_entity()
                    .with(snake_sprite, &mut sprites)
                    .with(GlobalTransform::default(),&mut gtransforms)
                    .with(Transform::from(pos),&mut transforms)
                    .with(Segment::body(seg_dir, snake.score), &mut segments)
                    .build();
        }
        
        if snake.food_available == false {
            snake.food_available = true;

            let transform = Transform::default();
            
            // Does this need refactoring - ?
            let transform = if let None = transforms.join().find(|t| t.translation == transform.translation) {
                generate_random_transform(dimns.width(),dimns.height())
            } else {
                return;
            };

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
            
        }

    }
}

pub fn generate_random_transform(width: f32,height: f32) -> Transform {
    let x = (rand::thread_rng().gen_range(0,width as u32) as f32 / 8.0).floor() * 8.0;
    let y = (rand::thread_rng().gen_range(0,height as u32) as f32 / 8.0).floor() * 8.0;
    Transform::from(Vector3::new(x,y,0.0))
}