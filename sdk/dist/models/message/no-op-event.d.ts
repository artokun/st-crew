import * as flatbuffers from 'flatbuffers';
export declare class NoOpEvent implements flatbuffers.IUnpackableObject<NoOpEventT> {
  bb: flatbuffers.ByteBuffer | null;
  bb_pos: number;
  __init(i: number, bb: flatbuffers.ByteBuffer): NoOpEvent;
  static getRootAsNoOpEvent(bb: flatbuffers.ByteBuffer, obj?: NoOpEvent): NoOpEvent;
  static getSizePrefixedRootAsNoOpEvent(bb: flatbuffers.ByteBuffer, obj?: NoOpEvent): NoOpEvent;
  static startNoOpEvent(builder: flatbuffers.Builder): void;
  static endNoOpEvent(builder: flatbuffers.Builder): flatbuffers.Offset;
  static createNoOpEvent(builder: flatbuffers.Builder): flatbuffers.Offset;
  unpack(): NoOpEventT;
  unpackTo(_o: NoOpEventT): void;
}
export declare class NoOpEventT implements flatbuffers.IGeneratedObject {
  constructor();
  pack(builder: flatbuffers.Builder): flatbuffers.Offset;
}
