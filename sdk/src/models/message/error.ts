// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



export class error implements flatbuffers.IUnpackableObject<errorT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):error {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAserror(bb:flatbuffers.ByteBuffer, obj?:error):error {
  return (obj || new error()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAserror(bb:flatbuffers.ByteBuffer, obj?:error):error {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new error()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static starterror(builder:flatbuffers.Builder) {
  builder.startObject(0);
}

static enderror(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createerror(builder:flatbuffers.Builder):flatbuffers.Offset {
  error.starterror(builder);
  return error.enderror(builder);
}

unpack(): errorT {
  return new errorT();
}


unpackTo(_o: errorT): void {}
}

export class errorT implements flatbuffers.IGeneratedObject {
constructor(){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  return error.createerror(builder);
}
}