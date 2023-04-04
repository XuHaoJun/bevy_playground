use std::ops::RangeInclusive;

use bevy::prelude::*;

use crate::constants::BrickType;

#[derive(Resource)]
pub struct FloorStageSpawner {
    pub timer: Timer,
    pub(crate) prob: BrickProbability,
}

pub struct BrickProbability {
    pub(crate) all: RangeInclusive<u32>,

    pub(crate) normal: RangeInclusive<u32>,
    pub(crate) fake: RangeInclusive<u32>,
    pub(crate) nails: RangeInclusive<u32>,
    pub(crate) conveyor: RangeInclusive<u32>,
    pub(crate) spring: RangeInclusive<u32>,
}

impl BrickProbability {
    fn new() -> Self {
        Self {
            all: RangeInclusive::new(1, 100),

            normal: RangeInclusive::new(1, 50),
            fake: RangeInclusive::new(51, 60),
            nails: RangeInclusive::new(61, 80),
            conveyor: RangeInclusive::new(81, 90),
            spring: RangeInclusive::new(91, 100),
        }
    }

    fn sample(&self, rng: &mut fastrand::Rng) -> BrickType {
        let n = rng.u32(self.all.clone());
        let ranges = vec![
            (BrickType::Normal, &self.normal),
            (BrickType::Fake, &self.fake),
            (BrickType::Nails, &self.nails),
            (BrickType::Conveyor, &self.conveyor),
            (BrickType::Spring, &self.spring),
        ];
        ranges.iter().find(|x| x.1.contains(&n)).unwrap().0
    }
}

impl Default for FloorStageSpawner {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(15.0, TimerMode::Repeating),
            prob: BrickProbability::new(),
        }
    }
}
