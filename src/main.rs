use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(
        ImagePlugin::default_nearest()), 
        PhysicsPlugins::default()))
        .add_systems(Startup, (setup_scene, setup_player))
        .add_systems(Update, player_input)
        .run();
}

#[derive(Component)]
struct Player {
    pub can_jump: bool,
}

#[derive(Component)]
struct Score {
    pub value: u32,
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());

    // Create the ground
    commands
    .spawn(Collider::cuboid(1000.0, 50.0))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)))
    .insert(SpriteBundle {
        texture: asset_server.load("dino.png"),
        ..default()
    });
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create the dino
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(10.0, 12.5))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)))
        .insert(SpriteBundle {
            texture: asset_server.load("dino.png"),
            transform: Transform::from_xyz(-500., 0., 500.).with_scale(Vec3::splat(3.)),
            ..default()
        })
        .insert(GravityScale(60.0));
}

fn player_input(mut query: Query<&mut LinearVelocity>, mut keyboard_input: ResMut<Input<KeyCode>>){
    // here we will check if we pressed space and if we did we will add a velocity to the player
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut linear_velocity in query.iter_mut() {
            linear_velocity.y += 500.0;
        }
        print!("jumped");
    }
}