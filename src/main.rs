// src/main.rs
use bevy::prelude::*;

fn main(){
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Misspelled".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d::default());
    /*
    let texture = asset_server.load("characters_atlas.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 32), 9, 10, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    */

    commands.spawn((
        Sprite::from_image(asset_server.load("character.png")),
        Transform::from_scale(Vec3::new(5.0, 5.0, 1.0)),

    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _) in characters.iter_mut() {
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += 100.0 * time.delta_secs();
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= 100.0 * time.delta_secs();
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 100.0 * time.delta_secs();
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += 100.0 * time.delta_secs();
        }
    }
}