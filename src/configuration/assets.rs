use bevy::prelude::*;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "new_asset_loader")]
        {
            info!("new asset loader");
            crate::assets::AssetLoadingPlugin.build(app)
        }
    }
}
pub use _assets::*;

#[cfg(feature = "new_asset_loader")]
pub mod _assets {
    pub use crate::assets::*;
}
