import * as flatbuffers from 'flatbuffers';
export declare class ServerStatEvent implements flatbuffers.IUnpackableObject<ServerStatEventT> {
  bb: flatbuffers.ByteBuffer | null;
  bb_pos: number;
  __init(i: number, bb: flatbuffers.ByteBuffer): ServerStatEvent;
  static getRootAsServerStatEvent(bb: flatbuffers.ByteBuffer, obj?: ServerStatEvent): ServerStatEvent;
  static getSizePrefixedRootAsServerStatEvent(bb: flatbuffers.ByteBuffer, obj?: ServerStatEvent): ServerStatEvent;
  clientsConnected(): number;
  static startServerStatEvent(builder: flatbuffers.Builder): void;
  static addClientsConnected(builder: flatbuffers.Builder, clientsConnected: number): void;
  static endServerStatEvent(builder: flatbuffers.Builder): flatbuffers.Offset;
  static createServerStatEvent(builder: flatbuffers.Builder, clientsConnected: number): flatbuffers.Offset;
  unpack(): ServerStatEventT;
  unpackTo(_o: ServerStatEventT): void;
}
export declare class ServerStatEventT implements flatbuffers.IGeneratedObject {
  clientsConnected: number;
  constructor(clientsConnected?: number);
  pack(builder: flatbuffers.Builder): flatbuffers.Offset;
}
