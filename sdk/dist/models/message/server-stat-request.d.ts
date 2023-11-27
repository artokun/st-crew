import * as flatbuffers from 'flatbuffers';
export declare class ServerStatRequest implements flatbuffers.IUnpackableObject<ServerStatRequestT> {
  bb: flatbuffers.ByteBuffer | null;
  bb_pos: number;
  __init(i: number, bb: flatbuffers.ByteBuffer): ServerStatRequest;
  static getRootAsServerStatRequest(bb: flatbuffers.ByteBuffer, obj?: ServerStatRequest): ServerStatRequest;
  static getSizePrefixedRootAsServerStatRequest(bb: flatbuffers.ByteBuffer, obj?: ServerStatRequest): ServerStatRequest;
  static startServerStatRequest(builder: flatbuffers.Builder): void;
  static endServerStatRequest(builder: flatbuffers.Builder): flatbuffers.Offset;
  static createServerStatRequest(builder: flatbuffers.Builder): flatbuffers.Offset;
  unpack(): ServerStatRequestT;
  unpackTo(_o: ServerStatRequestT): void;
}
export declare class ServerStatRequestT implements flatbuffers.IGeneratedObject {
  constructor();
  pack(builder: flatbuffers.Builder): flatbuffers.Offset;
}
