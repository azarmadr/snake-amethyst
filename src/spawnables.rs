use amethyst::{
    core::{
        cgmath::Vector3,
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle,ScreenDimensions},
};

pub struct Food(bool);


impl Default for Food {
    fn default() -> Self {
        Food(false)
    }
}

impl Component for Food {
    type Storage = DenseVecStorage<Self>;
}

