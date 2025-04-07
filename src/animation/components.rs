use bevy::{platform_support::collections::HashMap, prelude::*};
use serde::Deserialize;

use crate::ai::state::{ActionState, FacingDirection};

#[derive(Clone, Debug, Component)]
pub enum AnimationIndices {
    Cycle(std::iter::Cycle<std::ops::RangeInclusive<usize>>),
    OneShot(std::ops::RangeInclusive<usize>),
}
impl AnimationIndices {
    pub fn start(&self) -> usize {
        match self {
            // NOTE: this is not perfect, there's not easy way to access the original iterator
            // start which is what I would want.
            // TODO: Create helper functions to instantiate AnimationIndices types, that way it's
            // easier to include metadata
            AnimationIndices::Cycle(cycle) => cycle.clone().next().unwrap_or_default(),
            AnimationIndices::OneShot(range_inclusive) => *range_inclusive.start(),
        }
    }
}

impl Default for AnimationIndices {
    fn default() -> Self {
        Self::OneShot(0..=0)
    }
}

#[derive(Component, Deref, DerefMut, Default)]
#[require(AnimationIndices)]
pub struct AnimationTimer(pub Timer);

use config_macros::RonDefault;
#[derive(Resource, Clone, Deserialize, RonDefault)]
#[ron("src/configuration/properties/animations.ron")]
pub struct DefaultAnimationConfig {
    pub sprite_size: UVec2,
    pub columns: usize,
    pub rows: usize,
    pub animations: HashMap<(ActionState, FacingDirection), AnimationData>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AnimationData {
    pub row: usize,          // Row in the sprite sheet
    pub frame_count: usize,  // Number of frames in animation
    pub frame_duration: f32, // Duration of each frame
}

impl DefaultAnimationConfig {
    pub fn get_animation(&self, state: ActionState, direction: FacingDirection) -> &AnimationData {
        self.animations.get(&(state, direction)).unwrap_or_else(|| {
            panic!(
                "Missing animation data for {:?} {:?}",
                state.clone(),
                direction.clone()
            )
        })
    }

    pub fn get_indices(&self, state: ActionState, direction: FacingDirection) -> AnimationIndices {
        let animation = self.get_animation(state, direction);
        let first = animation.row * self.columns;
        let last = first + animation.frame_count - 1;
        AnimationIndices::Cycle((first..=last).cycle())
    }

    pub fn get_timer(&self, state: ActionState, direction: FacingDirection) -> Timer {
        let animation = self.get_animation(state, direction);
        Timer::from_seconds(animation.frame_duration, TimerMode::Repeating)
    }
}
