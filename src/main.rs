
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component, Resource)]
enum Direction {
    Up,
    Down,
}

#[derive(Component)]
struct Jump {
    pub velocity: f32,
    pub can_jump: bool,
}

#[derive(Component)]
struct Score {
    pub value: u32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("dino.png"),
            transform: Transform::from_xyz(-500., 0., 500.).with_scale(Vec3::splat(5.)),
            ..default()
        },
        Direction::Up,

    )).insert(Jump { velocity: 0.0, can_jump: true});
}

fn sprite_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Jump, &mut Transform)>,
) {
    for (mut jump, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) && jump.can_jump {
            // Apply jump force
            jump.velocity = 600.0;
            jump.can_jump = false;
        }

        // Apply gravity
        jump.velocity -= 1200.0 * time.delta_seconds();

        // Move sprite
        transform.translation.y += jump.velocity * time.delta_seconds();

        // Stop falling at ground level
        if transform.translation.y < -300.0 {
            transform.translation.y = -300.0;
            jump.velocity = 0.0;
            jump.can_jump = true;
        }
    }
}