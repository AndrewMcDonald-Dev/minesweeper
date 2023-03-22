use bevy::{prelude::*, window::PresentMode};
use board_plugin::resources::{BoardAssets, BoardOptions, SpriteMaterial};
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
        safe_start: true,
        ..Default::default()
    });

    app.add_startup_system(board_setup);

    app.add_plugin(BoardPlugin);

    // Startup system (cameras)
    app.add_startup_system(camera_setup);

    // Run the app
    app.run();
}

fn board_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Board assets
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..Default::default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::GRAY,
            ..Default::default()
        },
        bomb_counter_font: asset_server.load("fonts/ComicCode-Regular.otf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: SpriteMaterial {
            color: Color::WHITE,
            texture: asset_server.load("sprites/flag.png"),
        },
        bomb_material: SpriteMaterial {
            texture: asset_server.load("sprites/bomb.png"),
            color: Color::WHITE,
        },
    })
}

fn camera_setup(mut commands: Commands) {
    //2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}
