use super::WsMessage;

pub trait WsMessageType {
    fn to_message(self) -> WsMessage;
}
