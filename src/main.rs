use bevy::{prelude::*, window::PresentMode};
use board_plugin::resources::BoardOptions;
use board_plugin::BoardPlugin;

fn main() {
    let mut binding = App::new();
    let app = binding
        //window setup
        //Bevy default plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mine Sweeper!".to_string(),
                resolution: (700., 800.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }));

    app.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.0,
        ..Default::default()
    })
    .add_plugin(BoardPlugin);

    // Startup system (cameras)
    app.add_startup_system(camera_setup);

    // Run the app
    app.run();
}

fn camera_setup(mut commands: Commands) {
    //2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}
