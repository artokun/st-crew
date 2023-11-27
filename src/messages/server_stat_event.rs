use crate::generated::message::{GetServer, GetServerArgs, Message, MessageArgs, MessageType};

pub fn buffer(clients_connected: u16) -> Vec<u8> {
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1);
    let event = GetServer::create(&mut builder, &GetServerArgs { clients_connected });
    let message = Message::create(
        &mut builder,
        &MessageArgs {
            message_type: MessageType::GetServer,
            message: Some(event.as_union_value()),
        },
    );
    builder.finish(message, None);
    builder.finished_data().to_vec()
}

#[cfg(test)]
mod tests {
    use crate::generated::message::root_as_message;

    use super::*;

    #[test]
    fn test_buffer() {
        let clients_connected = 10;
        let buf = buffer(clients_connected);

        let message = root_as_message(&buf).unwrap();
        let event = message.message_as_get_server().unwrap();
        assert_eq!(event.clients_connected(), clients_connected);
    }
}
