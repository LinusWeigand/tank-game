use bevy::{ecs::query, input::keyboard, prelude::*, sprite::MaterialMesh2dBundle, transform::commands};


const BALL_COLOR: Color = Color::srgb(0.5, 1.0, 1.0);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_DIAMETER: f32 = 30.;
const BALL_SPEED: f32 = 400.0;

const TANK_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const TANK_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(TankerPlugin)
    .run();
}

pub struct TankerPlugin;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Tank;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);



#[derive(Resource)]
pub struct TankerMoveTimer(Timer);

impl Plugin for TankerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TankerMoveTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, setup);
        app.add_systems(FixedUpdate, (apply_velocity, move_tank, shoot));
    }
}

fn setup(
    mut commands: Commands, 
    mut meshes:  ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
    ) {
    //Camera
    commands.spawn(Camera2dBundle::default());

   

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            material: materials.add(TANK_COLOR),
            transform: Transform::from_translation(TANK_STARTING_POSITION)
                .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
            ..default()
        },
        Tank,
    ));
}


fn move_tank(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Tank>>,
    time: Res<Time>
) {
    let mut tank_transform = query.single_mut();
    let mut rotation_direction = 0.;
    let mut direction = tank_transform.rotation * Vec3::X;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        rotation_direction += 1.;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        rotation_direction -= 1.;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        tank_transform.translation += direction * 2.;
    }

    tank_transform.rotation *= Quat::from_rotation_z(0.05 * rotation_direction);
}

fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&Transform, With<Tank>>,
) {

    let tank_transform = query.single_mut();
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(BALL_COLOR),
                transform: Transform::from_translation(tank_transform.translation)
                    .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
                ..default()
            },
        Ball,
        Velocity((tank_transform.rotation * Vec3::X).truncate() * BALL_SPEED),
        ));
    }
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}