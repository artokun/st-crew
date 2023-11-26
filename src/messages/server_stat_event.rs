
use async_tungstenite::tungstenite::protocol::Message as WSMessage;
use crate::generated::message_generated::message::{
    Message, MessageArgs, MessageType, ServerStatEvent, ServerStatEventArgs,
};

pub fn buffer(clients_connected: u32) -> WSMessage {
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
    let event = ServerStatEvent::create(&mut builder, &ServerStatEventArgs { clients_connected });
    let message = Message::create(
        &mut builder,
        &MessageArgs {
            message_type: MessageType::ServerStatEvent,
            message: Some(event.as_union_value()),
            ..Default::default()
        },
    );
    builder.finish(message, None);
    WSMessage::Binary(builder.finished_data().to_vec())
}