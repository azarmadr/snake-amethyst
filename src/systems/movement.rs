use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,Resources,WriteExpect};
use amethyst::core::transform::Transform;
use amethyst::core::cgmath::Vector3;
use amethyst::core::timing::{Stopwatch};

use snake::{Segment,SegmentType,SegmentDirection,Snake};

use std::time::{Duration,Instant};

pub struct SnakeMovementSystem {
    pub tick: Stopwatch,
    pub tick_period: Duration
}
impl Default for SnakeMovementSystem {
    fn default() -> Self {
        Self {
            tick: Stopwatch::new(),
            tick_period: Duration::from_millis( ((1.0/6.0) * 1000.0) as u64 ) ,
        }
    }
}

impl<'s> System<'s> for SnakeMovementSystem {
    type SystemData = (
        WriteStorage<'s,Transform>,
        WriteStorage<'s, Segment>,
        WriteExpect<'s, Snake>,
    );
    fn setup(&mut self, _res: &mut Resources) {
        self.tick = Stopwatch::Started(Duration::from_millis(0), Instant::now());
    }
    fn run(&mut self, (mut transforms,mut segments,mut snake) : Self::SystemData) {
        let period = Duration::from_millis((snake.score * 5) as u64);
        let current_pos = if self.tick.elapsed() > ( self.tick_period - period ){
            let (transf,head_segment) = (&mut transforms, &segments).join().find(|(_,s)| s.t == SegmentType::Head).unwrap();
            snake.last_head_pos = transf.translation;
            snake.last_head_dir = head_segment.direction;
            transf.translation += match head_segment.direction {
                SegmentDirection::Up => Vector3::new(0.0,8.0,0.0),
                SegmentDirection::Left => Vector3::new(-8.0,0.0,0.0),
                SegmentDirection::Down => Vector3::new(0.0,-8.0,0.0),
                SegmentDirection::Right => Vector3::new(8.0,0.0,0.0),
                _ => Vector3::new(0.0,0.0,0.0),
            };
            self.tick.restart();
            transf.translation
        } else {
            snake.last_head_pos
        };

        if snake.last_head_pos != current_pos {
            let dirs = {
                let mut segs: Vec<_> = (segments).join().filter(|s| s.t == SegmentType::Body).collect();
                segs.sort_by_key(|s| s.id);
                segs.iter_mut().map(|s| (s.id, s.direction)).collect::<Vec<_>>()
            };
            let mut custom_vec: Vec<_> = (&mut transforms, &mut segments).join().filter(|(_,s)| s.t == SegmentType::Body).collect();
            custom_vec.sort_by_key(|(_,s)| s.id);
            for (idx,(trans, seg)) in custom_vec.iter_mut().enumerate() {
                trans.translation += match seg.direction {
                    SegmentDirection::Up => Vector3::new(0.0,8.0,0.0),
                    SegmentDirection::Left => Vector3::new(-8.0,0.0,0.0),
                    SegmentDirection::Down => Vector3::new(0.0,-8.0,0.0),
                    SegmentDirection::Right => Vector3::new(8.0,0.0,0.0),
                    _ => Vector3::new(0.0,0.0,0.0),
                };
                if idx == 0 {
                    seg.direction = snake.last_head_dir;
                } else {
                    seg.direction = dirs[idx-1].1; 
                }
            }

        }
        

    }
}