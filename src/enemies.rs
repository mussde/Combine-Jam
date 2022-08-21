use bevy::{prelude::*, ecs::system::Spawn};

pub struct EnemyPlugin;

const YPOS: f32 = 11.0;

#[derive(Component)]
struct Enemy;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

struct SpawnEnemyEvent;


impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app 
            .add_event::<SpawnEnemyEvent>()

            .add_startup_system(spawn_enemy)

            .add_system(animate_sprite)
            
            .add_system(control)
        ;
    }
}


fn control(
    keys: Res<Input<KeyCode>>,
    mut spawn_event: EventWriter<SpawnEnemyEvent>,
) {
    if keys.just_pressed(KeyCode::Space) {
        spawn_event.send(SpawnEnemyEvent);
    }
}



fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut spawn_event: EventReader<SpawnEnemyEvent>,
) {
    if spawn_event.iter().is_empty() {
        return;
    }

    let texture_handle = asset_server.load("Slimeman.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 8, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3 { x: 32.0, y: YPOS, z: 1.0 },
                ..default()
            },
            ..default()
        })
        .insert(Enemy)
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))

    ;
}


fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    ), With<Enemy>>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
