import * as flatbuffers from 'flatbuffers';
export declare class Vec2 implements flatbuffers.IUnpackableObject<Vec2T> {
  bb: flatbuffers.ByteBuffer | null;
  bb_pos: number;
  __init(i: number, bb: flatbuffers.ByteBuffer): Vec2;
  x(): number;
  y(): number;
  static sizeOf(): number;
  static createVec2(builder: flatbuffers.Builder, x: number, y: number): flatbuffers.Offset;
  unpack(): Vec2T;
  unpackTo(_o: Vec2T): void;
}
export declare class Vec2T implements flatbuffers.IGeneratedObject {
  x: number;
  y: number;
  constructor(x?: number, y?: number);
  pack(builder: flatbuffers.Builder): flatbuffers.Offset;
}
