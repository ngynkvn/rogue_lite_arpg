use bevy::prelude::*;

use baba_yaga::configuration::plugins::NativePlugins;

fn main() {
    let mut app = App::new();
    app.add_plugins(NativePlugins).add_plugins((
        #[cfg(feature = "debug")]
        {
            crate::debug::DebugPlugin
        },
        #[cfg(feature = "dump")]
        {
            bevy_mod_debugdump::CommandLineArgs
        },
    ));
    app.run();
}
