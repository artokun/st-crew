// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

export class MOTDEvent implements flatbuffers.IUnpackableObject<MOTDEventT> {
  bb: flatbuffers.ByteBuffer | null = null;
  bb_pos = 0;
  __init(i: number, bb: flatbuffers.ByteBuffer): MOTDEvent {
    this.bb_pos = i;
    this.bb = bb;
    return this;
  }

  static getRootAsMOTDEvent(bb: flatbuffers.ByteBuffer, obj?: MOTDEvent): MOTDEvent {
    return (obj || new MOTDEvent()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
  }

  static getSizePrefixedRootAsMOTDEvent(bb: flatbuffers.ByteBuffer, obj?: MOTDEvent): MOTDEvent {
    bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
    return (obj || new MOTDEvent()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
  }

  title(): string | null;
  title(optionalEncoding: flatbuffers.Encoding): string | Uint8Array | null;
  title(optionalEncoding?: any): string | Uint8Array | null {
    const offset = this.bb!.__offset(this.bb_pos, 4);
    return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
  }

  body(): string | null;
  body(optionalEncoding: flatbuffers.Encoding): string | Uint8Array | null;
  body(optionalEncoding?: any): string | Uint8Array | null {
    const offset = this.bb!.__offset(this.bb_pos, 6);
    return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
  }

  static startMOTDEvent(builder: flatbuffers.Builder) {
    builder.startObject(2);
  }

  static addTitle(builder: flatbuffers.Builder, titleOffset: flatbuffers.Offset) {
    builder.addFieldOffset(0, titleOffset, 0);
  }

  static addBody(builder: flatbuffers.Builder, bodyOffset: flatbuffers.Offset) {
    builder.addFieldOffset(1, bodyOffset, 0);
  }

  static endMOTDEvent(builder: flatbuffers.Builder): flatbuffers.Offset {
    const offset = builder.endObject();
    return offset;
  }

  static createMOTDEvent(
    builder: flatbuffers.Builder,
    titleOffset: flatbuffers.Offset,
    bodyOffset: flatbuffers.Offset
  ): flatbuffers.Offset {
    MOTDEvent.startMOTDEvent(builder);
    MOTDEvent.addTitle(builder, titleOffset);
    MOTDEvent.addBody(builder, bodyOffset);
    return MOTDEvent.endMOTDEvent(builder);
  }

  unpack(): MOTDEventT {
    return new MOTDEventT(this.title(), this.body());
  }

  unpackTo(_o: MOTDEventT): void {
    _o.title = this.title();
    _o.body = this.body();
  }
}

export class MOTDEventT implements flatbuffers.IGeneratedObject {
  constructor(
    public title: string | Uint8Array | null = null,
    public body: string | Uint8Array | null = null
  ) {}

  pack(builder: flatbuffers.Builder): flatbuffers.Offset {
    const title = this.title !== null ? builder.createString(this.title!) : 0;
    const body = this.body !== null ? builder.createString(this.body!) : 0;

    return MOTDEvent.createMOTDEvent(builder, title, body);
  }
}
