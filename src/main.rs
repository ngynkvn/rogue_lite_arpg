use bevy::prelude::*;

use baba_yaga::configuration::plugins::NativePlugins;

fn main() {
    let mut app = App::new();
    app.add_plugins(NativePlugins).add_plugins((
        #[cfg(feature = "debugdump")]
        {
            bevy_mod_debugdump::CommandLineArgs
        },
    ));

    #[cfg(feature = "debug")]
    {
        baba_yaga::debug::DebugPlugin.build(&mut app);
        return;
    }
    app.run();
}
