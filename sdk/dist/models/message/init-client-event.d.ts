import * as flatbuffers from 'flatbuffers';
import { Client, ClientT } from '../client/client.js';
export declare class InitClientEvent implements flatbuffers.IUnpackableObject<InitClientEventT> {
  bb: flatbuffers.ByteBuffer | null;
  bb_pos: number;
  __init(i: number, bb: flatbuffers.ByteBuffer): InitClientEvent;
  static getRootAsInitClientEvent(bb: flatbuffers.ByteBuffer, obj?: InitClientEvent): InitClientEvent;
  static getSizePrefixedRootAsInitClientEvent(bb: flatbuffers.ByteBuffer, obj?: InitClientEvent): InitClientEvent;
  client(obj?: Client): Client | null;
  static startInitClientEvent(builder: flatbuffers.Builder): void;
  static addClient(builder: flatbuffers.Builder, clientOffset: flatbuffers.Offset): void;
  static endInitClientEvent(builder: flatbuffers.Builder): flatbuffers.Offset;
  static createInitClientEvent(builder: flatbuffers.Builder, clientOffset: flatbuffers.Offset): flatbuffers.Offset;
  unpack(): InitClientEventT;
  unpackTo(_o: InitClientEventT): void;
}
export declare class InitClientEventT implements flatbuffers.IGeneratedObject {
  client: ClientT | null;
  constructor(client?: ClientT | null);
  pack(builder: flatbuffers.Builder): flatbuffers.Offset;
}
