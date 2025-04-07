use bevy::{
    prelude::*,
    scene::ron::{self},
};
use config_macros::DefaultRon;
use serde::Deserialize;

use crate::{
    ai::SimpleMotion,
    enemy::{EnemyAssets, Experience},
    map::components::InstanceAssets,
    player::PlayerStats,
    progression::GameProgress,
};

pub struct PropertiesPlugin;

impl Plugin for PropertiesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_ron::<InstanceAssets>(include_bytes!("properties/instances.ron"))
            .insert_ron::<EnemyAssets>(include_bytes!("properties/enemies.ron"));
    }
}

#[derive(DefaultRon, Clone, Deserialize)]
#[ron("src/configuration/properties/general.ron")]
struct GeneralConfig(GameProgress, PlayerStats, Experience, SimpleMotion);

impl Default for GameProgress {
    fn default() -> Self {
        GeneralConfig__RON_DERIVED_DEFAULT__.0.clone()
    }
}
impl Default for PlayerStats {
    fn default() -> Self {
        GeneralConfig__RON_DERIVED_DEFAULT__.1.clone()
    }
}

impl Default for Experience {
    fn default() -> Self {
        GeneralConfig__RON_DERIVED_DEFAULT__.2.clone()
    }
}

impl Default for SimpleMotion {
    fn default() -> Self {
        GeneralConfig__RON_DERIVED_DEFAULT__.3.clone()
    }
}

/// This trait is used to load properties from a RON file.
/// The RON file is included in the binary using the `include_bytes!` macro.
trait RonResourceExt {
    fn insert_ron<'de, T>(&mut self, data: &'static [u8]) -> &mut Self
    where
        T: Deserialize<'de> + Resource;
}
impl RonResourceExt for App {
    fn insert_ron<'de, T>(&mut self, data: &'static [u8]) -> &mut Self
    where
        T: Deserialize<'de> + Resource,
    {
        let data =
            ron::de::from_bytes::<T>(data).expect("failed to load properties from provided path");
        self.insert_resource(data);
        self
    }
}

// NOTE: Alternative way to load from RON is to have `impl RonData` for each struct
// and use `ron::de::from_bytes::<T>(T::DATA)` instead of `include_bytes!`.
#[allow(dead_code)]
trait RonData {
    #[allow(non_snake_case)]
    const DATA: &'static [u8];
}

#[allow(dead_code)]
trait InitRonResourceExt {
    fn init_ron<'de, T>(&mut self) -> &mut Self
    where
        T: Deserialize<'de> + Resource + RonData;
}

impl InitRonResourceExt for App {
    fn init_ron<'de, T>(&mut self) -> &mut Self
    where
        T: Deserialize<'de> + Resource + RonData,
    {
        let data = ron::de::from_bytes::<T>(T::DATA)
            .expect("failed to load properties from provided path");
        self.insert_resource(data);
        self
    }
}
