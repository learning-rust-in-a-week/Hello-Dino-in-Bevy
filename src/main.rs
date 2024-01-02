use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup_graphics, setup_physics))
        .add_systems(Update, print_ball_altitude)
        .run();
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

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands, asset_server: Res<AssetServer>) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(1000.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(10.0, 12.5))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)))
        .insert(SpriteBundle {
            texture: asset_server.load("dino.png"),
            transform: Transform::from_xyz(-500., 0., 500.).with_scale(Vec3::splat(2.)),
            ..default()
        });
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
 
    }
}