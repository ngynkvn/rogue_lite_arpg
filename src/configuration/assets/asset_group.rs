use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
};

pub trait AssetGroup {
    fn hello_macro();
    fn load_assets_system(commands: Commands, server: Res<AssetServer>);
}
