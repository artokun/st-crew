import * as flatbuffers from 'flatbuffers';
export declare class error implements flatbuffers.IUnpackableObject<errorT> {
  bb: flatbuffers.ByteBuffer | null;
  bb_pos: number;
  __init(i: number, bb: flatbuffers.ByteBuffer): error;
  static getRootAserror(bb: flatbuffers.ByteBuffer, obj?: error): error;
  static getSizePrefixedRootAserror(bb: flatbuffers.ByteBuffer, obj?: error): error;
  static starterror(builder: flatbuffers.Builder): void;
  static enderror(builder: flatbuffers.Builder): flatbuffers.Offset;
  static createerror(builder: flatbuffers.Builder): flatbuffers.Offset;
  unpack(): errorT;
  unpackTo(_o: errorT): void;
}
export declare class errorT implements flatbuffers.IGeneratedObject {
  constructor();
  pack(builder: flatbuffers.Builder): flatbuffers.Offset;
}
