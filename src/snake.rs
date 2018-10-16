use amethyst::{
    core::{
        cgmath::Vector3,
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle,ScreenDimensions},
};
use rand;
use rand::Rng;



#[derive(PartialEq,Eq,Debug)]
pub enum SegmentType {
    Head,
    Body,
}

#[derive(Debug)]
pub enum SegmentDirection {
    Left,
    Right,
    Up,
    Down,
    Idle,
}

#[derive(Debug)]
pub struct Segment{
    pub t: SegmentType,
    pub direction: SegmentDirection,
    pub id: u32,
}
impl Segment {
    pub fn body(direction: SegmentDirection,id: u32) -> Self {
        Segment {
            t: SegmentType::Body,
            direction: direction,
            id: id,
        }
    }
}
impl Default for Segment {
    fn default() -> Self {
        Segment {
            t: SegmentType::Head,
            direction: SegmentDirection::Idle,
            id: 0,
        }
    }
}
impl Component for Segment {
    type Storage = DenseVecStorage<Self>;
}

pub struct Snake(Vec<Segment>);

impl Default for Snake {
    fn default() -> Self {
        Snake(Vec::new())
    }
}


pub fn initialise_snake(world: &mut World,sheet_handle: SpriteSheetHandle){
    world.add_resource(Snake(Vec::new()));
    world.register::<Segment>();

    let snake_color_id = rand::thread_rng().gen_range(0,7);

    let snake_sprite = SpriteRender {
        sprite_sheet: sheet_handle,
        sprite_number: snake_color_id,
        flip_horizontal: false,
        flip_vertical: false,
    };

    let (width,height) = {
        let dimn = world.read_resource::<ScreenDimensions>();
        assert!(dimn.width() % 8.0 == 0.0, dimn.height() % 8.0 == 0.0);
        (dimn.width(), dimn.height())
    };

    let mut transform = Transform::default();
    
    let (x,y) = ((width / 16.0).round() * 8.0,(height / 16.0).round() * 8.0);
    
    transform.translation = Vector3::new(0.0,0.0,0.0);

    world.create_entity()
                .with(snake_sprite)
                .with(GlobalTransform::default())
                .with(transform)
                .with(Segment::default())
                .build();
}

