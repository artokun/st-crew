import * as flatbuffers from 'flatbuffers';
export declare class RequestServerStatEvent implements flatbuffers.IUnpackableObject<RequestServerStatEventT> {
  bb: flatbuffers.ByteBuffer | null;
  bb_pos: number;
  __init(i: number, bb: flatbuffers.ByteBuffer): RequestServerStatEvent;
  static getRootAsRequestServerStatEvent(
    bb: flatbuffers.ByteBuffer,
    obj?: RequestServerStatEvent
  ): RequestServerStatEvent;
  static getSizePrefixedRootAsRequestServerStatEvent(
    bb: flatbuffers.ByteBuffer,
    obj?: RequestServerStatEvent
  ): RequestServerStatEvent;
  static startRequestServerStatEvent(builder: flatbuffers.Builder): void;
  static endRequestServerStatEvent(builder: flatbuffers.Builder): flatbuffers.Offset;
  static createRequestServerStatEvent(builder: flatbuffers.Builder): flatbuffers.Offset;
  unpack(): RequestServerStatEventT;
  unpackTo(_o: RequestServerStatEventT): void;
}
export declare class RequestServerStatEventT implements flatbuffers.IGeneratedObject {
  constructor();
  pack(builder: flatbuffers.Builder): flatbuffers.Offset;
}
