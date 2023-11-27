import * as flatbuffers from 'flatbuffers';
import { Client, ClientT } from '../client/client.js';
import { ClientAction } from '../client/client-action.js';
export declare class ClientUpdateEvent implements flatbuffers.IUnpackableObject<ClientUpdateEventT> {
  bb: flatbuffers.ByteBuffer | null;
  bb_pos: number;
  __init(i: number, bb: flatbuffers.ByteBuffer): ClientUpdateEvent;
  static getRootAsClientUpdateEvent(bb: flatbuffers.ByteBuffer, obj?: ClientUpdateEvent): ClientUpdateEvent;
  static getSizePrefixedRootAsClientUpdateEvent(bb: flatbuffers.ByteBuffer, obj?: ClientUpdateEvent): ClientUpdateEvent;
  action(): ClientAction;
  client(obj?: Client): Client | null;
  static startClientUpdateEvent(builder: flatbuffers.Builder): void;
  static addAction(builder: flatbuffers.Builder, action: ClientAction): void;
  static addClient(builder: flatbuffers.Builder, clientOffset: flatbuffers.Offset): void;
  static endClientUpdateEvent(builder: flatbuffers.Builder): flatbuffers.Offset;
  unpack(): ClientUpdateEventT;
  unpackTo(_o: ClientUpdateEventT): void;
}
export declare class ClientUpdateEventT implements flatbuffers.IGeneratedObject {
  action: ClientAction;
  client: ClientT | null;
  constructor(action?: ClientAction, client?: ClientT | null);
  pack(builder: flatbuffers.Builder): flatbuffers.Offset;
}
