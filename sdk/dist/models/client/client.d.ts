import * as flatbuffers from 'flatbuffers';
export declare class Client implements flatbuffers.IUnpackableObject<ClientT> {
  bb: flatbuffers.ByteBuffer | null;
  bb_pos: number;
  __init(i: number, bb: flatbuffers.ByteBuffer): Client;
  static getRootAsClient(bb: flatbuffers.ByteBuffer, obj?: Client): Client;
  static getSizePrefixedRootAsClient(bb: flatbuffers.ByteBuffer, obj?: Client): Client;
  id(): string | null;
  id(optionalEncoding: flatbuffers.Encoding): string | Uint8Array | null;
  name(): string | null;
  name(optionalEncoding: flatbuffers.Encoding): string | Uint8Array | null;
  static startClient(builder: flatbuffers.Builder): void;
  static addId(builder: flatbuffers.Builder, idOffset: flatbuffers.Offset): void;
  static addName(builder: flatbuffers.Builder, nameOffset: flatbuffers.Offset): void;
  static endClient(builder: flatbuffers.Builder): flatbuffers.Offset;
  static finishClientBuffer(builder: flatbuffers.Builder, offset: flatbuffers.Offset): void;
  static finishSizePrefixedClientBuffer(builder: flatbuffers.Builder, offset: flatbuffers.Offset): void;
  static createClient(
    builder: flatbuffers.Builder,
    idOffset: flatbuffers.Offset,
    nameOffset: flatbuffers.Offset
  ): flatbuffers.Offset;
  unpack(): ClientT;
  unpackTo(_o: ClientT): void;
}
export declare class ClientT implements flatbuffers.IGeneratedObject {
  id: string | Uint8Array | null;
  name: string | Uint8Array | null;
  constructor(id?: string | Uint8Array | null, name?: string | Uint8Array | null);
  pack(builder: flatbuffers.Builder): flatbuffers.Offset;
}
