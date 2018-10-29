use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};

pub struct Food(pub bool);


impl Default for Food {
    fn default() -> Self {
        Food(false)
    }
}

impl Component for Food {
    type Storage = DenseVecStorage<Self>;
}

