import * as flatbuffers from 'flatbuffers';
export declare class connect implements flatbuffers.IUnpackableObject<connectT> {
  bb: flatbuffers.ByteBuffer | null;
  bb_pos: number;
  __init(i: number, bb: flatbuffers.ByteBuffer): connect;
  static getRootAsconnect(bb: flatbuffers.ByteBuffer, obj?: connect): connect;
  static getSizePrefixedRootAsconnect(bb: flatbuffers.ByteBuffer, obj?: connect): connect;
  static startconnect(builder: flatbuffers.Builder): void;
  static endconnect(builder: flatbuffers.Builder): flatbuffers.Offset;
  static createconnect(builder: flatbuffers.Builder): flatbuffers.Offset;
  unpack(): connectT;
  unpackTo(_o: connectT): void;
}
export declare class connectT implements flatbuffers.IGeneratedObject {
  constructor();
  pack(builder: flatbuffers.Builder): flatbuffers.Offset;
}
