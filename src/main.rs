use bevy::prelude::*;
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
            // HelloPlugin
        ))
        .add_systems(Startup, startup_socket_listener)
        .add_systems(Update, receive_message)
        .run();
}
