use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use eddy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: Level::INFO,
                    ..Default::default()
                }),
            GamePlugin,
            PlayerPlugin,
            StatusBoardPlugin,
        ))
        .run();
}
