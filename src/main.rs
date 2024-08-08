use bevy::prelude::*;
use eddy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            GamePlugin,
            PlayerPlugin,
        )) // prevents blurry sprites
        .run();
}
