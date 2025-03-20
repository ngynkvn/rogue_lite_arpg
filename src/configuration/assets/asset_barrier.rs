use bevy::prelude::*;
use std::{marker::PhantomData, sync::Arc};

pub trait Safe: 'static + Send + Sync {}
impl<T: 'static + Send + Sync> Safe for T {}

#[derive(Resource, Default, Clone, Deref)]
pub struct ArcGuard(Arc<()>);

#[derive(Resource, Default, Clone, Deref)]
pub struct AssetBarrier<T: Safe> {
    #[deref]
    inner: ArcGuard,
    _marker: PhantomData<T>,
}
impl<T: Safe> AssetBarrier<T> {
    /// Create an [`AssetBarrier`] with a [`AssetBarrierGuard`].
    pub fn new() -> AssetBarrier<T> {
        AssetBarrier {
            inner: ArcGuard::default(),
            _marker: PhantomData,
        }
    }
    pub fn assets_ready(guard: Res<AssetBarrier<T>>) -> bool {
        Arc::strong_count(&guard) == 1
    }
}
