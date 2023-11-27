import * as flatbuffers from 'flatbuffers';
export declare class disconnect implements flatbuffers.IUnpackableObject<disconnectT> {
  bb: flatbuffers.ByteBuffer | null;
  bb_pos: number;
  __init(i: number, bb: flatbuffers.ByteBuffer): disconnect;
  static getRootAsdisconnect(bb: flatbuffers.ByteBuffer, obj?: disconnect): disconnect;
  static getSizePrefixedRootAsdisconnect(bb: flatbuffers.ByteBuffer, obj?: disconnect): disconnect;
  static startdisconnect(builder: flatbuffers.Builder): void;
  static enddisconnect(builder: flatbuffers.Builder): flatbuffers.Offset;
  static createdisconnect(builder: flatbuffers.Builder): flatbuffers.Offset;
  unpack(): disconnectT;
  unpackTo(_o: disconnectT): void;
}
export declare class disconnectT implements flatbuffers.IGeneratedObject {
  constructor();
  pack(builder: flatbuffers.Builder): flatbuffers.Offset;
}
