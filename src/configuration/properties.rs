use std::sync::LazyLock;

use bevy::{
    prelude::*,
    scene::ron::{self},
};
use config_macros::LazyRon;
use serde::Deserialize;

use crate::{
    ai::SimpleMotion,
    animation::DefaultAnimationConfig,
    enemy::{EnemyAssets, Experience},
    map::components::InstanceAssets,
    player::{interact::InteractionZone, PlayerStats},
    progression::GameProgress,
};

#[derive(LazyRon, Clone, Deserialize)]
#[lazy("src/configuration/properties/general.ron")]
struct GeneralConfig(GameProgress, PlayerStats, Experience, SimpleMotion);

static GENERAL_CONFIG: LazyLock<GeneralConfig> = std::sync::LazyLock::new(|| {
    ron::de::from_bytes(include_bytes!("properties/general.ron")).unwrap()
});
static INSTANCE_ASSETS: LazyLock<InstanceAssets> = std::sync::LazyLock::new(|| {
    ron::de::from_bytes(include_bytes!("properties/instances.ron")).unwrap()
});
static ENEMY_ASSETS: LazyLock<EnemyAssets> = std::sync::LazyLock::new(|| {
    ron::de::from_bytes(include_bytes!("properties/enemies.ron")).unwrap()
});
static ANIMATION_CONFIG: LazyLock<DefaultAnimationConfig> = std::sync::LazyLock::new(|| {
    ron::de::from_bytes(include_bytes!("properties/animations.ron")).unwrap()
});

impl InteractionZone {
    pub const OPEN_CHEST: Self = Self::Square { length: 40.0 };
    pub const NPC: Self = Self::Circle { radius: 30.0 };
    pub const ITEM_PICKUP: Self = Self::Circle { radius: 25.0 };
}

pub struct PropertiesPlugin;

impl Plugin for PropertiesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<InstanceAssets>(INSTANCE_ASSETS.clone())
            .insert_resource::<EnemyAssets>(ENEMY_ASSETS.clone());
    }
}

impl Default for GameProgress {
    fn default() -> Self {
        GENERAL_CONFIG.0.clone()
    }
}
impl Default for PlayerStats {
    fn default() -> Self {
        GENERAL_CONFIG.1.clone()
    }
}

impl Default for Experience {
    fn default() -> Self {
        GENERAL_CONFIG.2.clone()
    }
}
impl Default for SimpleMotion {
    fn default() -> Self {
        GENERAL_CONFIG.3.clone()
    }
}
impl Default for DefaultAnimationConfig {
    fn default() -> Self {
        ANIMATION_CONFIG.clone()
    }
}
