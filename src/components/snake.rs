use amethyst::{
    ecs::{
        Component, 
        storage::{
            NullStorage,
            DenseVecStorage,
        },
    },
};


#[derive(Default,Debug)]
pub struct HeadSegment;

impl Component for HeadSegment {
    type Storage = NullStorage<Self>;
}

#[derive(Default,Debug)]
pub struct BodySegment;

impl Component for BodySegment {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default,Debug)]
pub struct TailSegment;

impl Component for TailSegment {
    type Storage = NullStorage<Self>;
}


/// Represent the direction the snake should move to.
#[derive(Debug,Clone,Copy)]
pub enum SegmentDirection {
    Left,
    Right,
    Up,
    Down,
    Idle,
}

impl Component for SegmentDirection {
    type Storage = DenseVecStorage<Self>;
}