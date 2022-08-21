use bevy::{prelude::*, render::texture::ImageSettings};


#[derive(Default)]
struct CameraSettings {
    current: f32,
    vertical_offset: f32,
}

#[derive(Component)]
struct Camera;


mod enemies;


fn main() {
    App::new()
        // Ressourcer
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05,)))
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            height: 512.0,
            width: 1024.0,
            ..default()
        })

        .insert_resource(CameraSettings {
            current: 8.0,
            vertical_offset: 56.0,
            ..default()
        })

        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(enemies::EnemyPlugin)
        
        // Systemer
        .add_startup_system(setup)
        .add_startup_system(spawn_ground)

        .add_system(control_camera)
        .add_system(apple_camera_settings)

        .run();
}


fn spawn_ground(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("grass0.png");

    let mut lim = 8;

    while lim > 0 {
        lim -= 1;

        
        commands.spawn_bundle(SpriteBundle {
            texture: texture.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 16.0 * lim as f32,
                    y: 0.0,
                    z: 0.0
                },
                ..default()
            },
            ..default()
        });
    }
}


fn control_camera(
    keys: Res<Input<KeyCode>>,
    mut cam: ResMut<CameraSettings>,
) {
    if keys.just_pressed(KeyCode::Right) {
        cam.vertical_offset += 8.;
    }
    if keys.just_pressed(KeyCode::Left) {
        cam.vertical_offset -= 8.;
    }
}

fn apple_camera_settings(
    cam: Res<CameraSettings>,
    mut query: Query<&mut Transform, With<Camera>>
) {
    let mut transform = query.get_single_mut().expect("Too many cameras");
    transform.scale = Vec3 { x: 1.0/cam.current, y: 1.0/cam.current, z: 1.0/cam.current };
    transform.translation = Vec3 {
        x: cam.vertical_offset,
        y: 24.0,
        z: 100.0
    };
}


fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        ..default()
    }).insert(Camera);
}