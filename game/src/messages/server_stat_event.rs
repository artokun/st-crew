use crate::{
    ecs::plugins::websocket::{WsMessage, WsMessageType},
    generated::message::{GetServer, GetServerArgs, Message, MessageArgs, MessageType},
};

pub struct GetServerMessage {
    pub clients_connected: u16,
}

impl WsMessageType for GetServerMessage {
    fn to_message(self) -> WsMessage {
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1);

        let event = GetServer::create(
            &mut builder,
            &GetServerArgs {
                clients_connected: self.clients_connected,
            },
        );

        let message = Message::create(
            &mut builder,
            &MessageArgs {
                message_type: MessageType::GetServer,
                message: Some(event.as_union_value()),
            },
        );

        builder.finish(message, None);

        WsMessage::Binary(builder.finished_data().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let clients_connected = 10;

        let WsMessage::Binary(bytes) = GetServerMessage { clients_connected }.to_message() else {
            panic!("Failed to convert to message");
        };

        let message = flatbuffers::root::<Message>(&bytes).unwrap();
        let event = message.message_as_get_server().unwrap();
        assert_eq!(event.clients_connected(), clients_connected);
    }
}
