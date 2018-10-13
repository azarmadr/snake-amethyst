use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        cgmath::{Matrix4, Vector3},
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        Camera, MaterialTextureSet, PngFormat, Projection, Sprite, SpriteRender, SpriteSheet,
        SpriteSheetHandle, Texture, TextureCoordinates, TextureMetadata,ScreenDimensions,
    },
};
use rand::Rng;

pub struct HeadSegment {
    direction: SegmentDirection,
}

impl Default for HeadSegment {
    fn default() -> Self {
        HeadSegment {
            direction: SegmentDirection::Up,
        }
    }
}

impl Component for HeadSegment {
    type Storage = DenseVecStorage<Self>;
}

pub struct BodySegment{
    direction: SegmentDirection,
}

impl Default for BodySegment {
    fn default() -> Self {
        BodySegment {
            direction: SegmentDirection::Up,
        }
    }
}

impl Component for BodySegment {
    type Storage = DenseVecStorage<Self>;
}

pub fn create_head(world: &mut World,sheet_handle: SpriteSheetHandle){
    world.register::<HeadSegment>();
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
    transform.translation = Vector3::new(width / 2.0, height / 2.0,0.0);

    world.create_entity()
                .with(snake_sprite)
                .with(GlobalTransform::default())
                .with(transform)
                .with(HeadSegment::default())
                .build();
}

pub enum SegmentDirection {
    Left,
    Right,
    Up,
    Down,
}
