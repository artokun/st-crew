use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ws_server::{ReceiveError, WsConnection, WsListener, WsPlugin};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn startup_socket_listener(listener: Res<WsListener>) {
    listener.listen("127.0.0.1:8080");
}

fn receive_message(mut commands: Commands, connections: Query<(Entity, &WsConnection)>) {
    for (entity, conn) in connections.iter() {
        loop {
            match conn.receive() {
                Ok(message) => {
                    conn.send(message);
                }
                Err(ReceiveError::Empty) => break,
                Err(ReceiveError::Closed) => {
                    commands.entity(entity).despawn();
                    break;
                }
            }
        }
    }
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Art".to_string())));
    commands.spawn((Person, Name("Miew".to_string())));
    commands.spawn((Person, Name("Arya".to_string())));
    commands.spawn((Person, Name("Archie".to_string())));
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

fn main() {
    App::new()
        // .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins((
            MinimalPlugins,
            WsPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            // HelloPlugin
        ))
        .add_systems(Startup, (startup_socket_listener, setup_physics))
        .add_systems(Update, (receive_message, print_ball_altitude))
        .run();
}
