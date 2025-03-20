use bevy::prelude::*;

use crate::labels::states::AppState;

pub trait AssetGroup {
    fn load_assets_system(commands: commands, server: res<assetserver>);
}

pub trait AppAssetExt {
    fn load_assets_group<T: AssetGroup>(&mut self) -> &mut Self;
}

#[derive(SubStates, Eq, Default, Hash, Clone, Copy, Debug, PartialEq)]
#[source(AppState = AppState::AssetLoading)]
pub enum AssetLoadingState {
    #[default]
    Loading,
    LoadingComplete,
}

impl AppAssetExt for App {
    fn load_assets_group<U: AssetGroup>(&mut self) -> &mut Self {
        self.add_sub_state::<AssetLoadingState>()
            .add_systems(OnEnter(AppState::AssetLoading), U::load_assets_system)
    }
}
