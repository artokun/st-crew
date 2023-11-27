import * as flatbuffers from 'flatbuffers';
import { ClientUpdateEventT } from '../message/client-update-event.js';
import { InitClientEventT } from '../message/init-client-event.js';
import { InitStateEventT } from '../message/init-state-event.js';
import { MessageType } from '../message/message-type.js';
import { NoOpEventT } from '../message/no-op-event.js';
import { RequestServerStatEventT } from '../message/request-server-stat-event.js';
import { ServerStatEventT } from '../message/server-stat-event.js';
export declare class Message implements flatbuffers.IUnpackableObject<MessageT> {
  bb: flatbuffers.ByteBuffer | null;
  bb_pos: number;
  __init(i: number, bb: flatbuffers.ByteBuffer): Message;
  static getRootAsMessage(bb: flatbuffers.ByteBuffer, obj?: Message): Message;
  static getSizePrefixedRootAsMessage(bb: flatbuffers.ByteBuffer, obj?: Message): Message;
  messageType(): MessageType;
  message<T extends flatbuffers.Table>(obj: any): any | null;
  static startMessage(builder: flatbuffers.Builder): void;
  static addMessageType(builder: flatbuffers.Builder, messageType: MessageType): void;
  static addMessage(builder: flatbuffers.Builder, messageOffset: flatbuffers.Offset): void;
  static endMessage(builder: flatbuffers.Builder): flatbuffers.Offset;
  static finishMessageBuffer(builder: flatbuffers.Builder, offset: flatbuffers.Offset): void;
  static finishSizePrefixedMessageBuffer(builder: flatbuffers.Builder, offset: flatbuffers.Offset): void;
  static createMessage(
    builder: flatbuffers.Builder,
    messageType: MessageType,
    messageOffset: flatbuffers.Offset
  ): flatbuffers.Offset;
  unpack(): MessageT;
  unpackTo(_o: MessageT): void;
}
export declare class MessageT implements flatbuffers.IGeneratedObject {
  messageType: MessageType;
  message:
    | ClientUpdateEventT
    | InitClientEventT
    | InitStateEventT
    | NoOpEventT
    | RequestServerStatEventT
    | ServerStatEventT
    | null;
  constructor(
    messageType?: MessageType,
    message?:
      | ClientUpdateEventT
      | InitClientEventT
      | InitStateEventT
      | NoOpEventT
      | RequestServerStatEventT
      | ServerStatEventT
      | null
  );
  pack(builder: flatbuffers.Builder): flatbuffers.Offset;
}
