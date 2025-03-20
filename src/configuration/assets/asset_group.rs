use std::marker::PhantomData;

use bevy::prelude::*;

use crate::labels::states::AppState;
pub trait AssetGroup {
    fn load_assets_system(commands: Commands, server: Res<AssetServer>);
}

pub trait AppAssetExt {
    fn load_assets_group<T: AssetGroup + 'static>(&mut self) -> &mut Self;
}

impl AppAssetExt for App {
    fn load_assets_group<U: AssetGroup + 'static>(&mut self) -> &mut Self {
        self.add_systems(OnEnter(AppState::AssetLoading), U::load_assets_system)
    }
}

#[derive(SubStates, Eq, Default, Hash, Clone, Copy, Debug, PartialEq)]
#[source(AppState = AppState::AssetLoading)]
pub enum AssetLoadingState {
    #[default]
    Loading,
    LoadingComplete,
}
