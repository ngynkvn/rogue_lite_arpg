use bevy::prelude::*;
use std::sync::Arc;

#[derive(Resource, Default, Clone, Deref)]
pub struct AssetBarrier(Arc<()>);
impl AssetBarrier {
    /// Create an [`AssetBarrier`] with a [`AssetBarrierGuard`].
    pub fn new() -> AssetBarrier {
        AssetBarrier(Arc::new(()))
    }
    pub fn assets_ready(guard: Res<AssetBarrier>) -> bool {
        Arc::strong_count(&guard) == 1
    }
}
