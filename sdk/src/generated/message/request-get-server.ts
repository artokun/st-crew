// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



export class RequestGetServer implements flatbuffers.IUnpackableObject<RequestGetServerT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):RequestGetServer {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsRequestGetServer(bb:flatbuffers.ByteBuffer, obj?:RequestGetServer):RequestGetServer {
  return (obj || new RequestGetServer()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsRequestGetServer(bb:flatbuffers.ByteBuffer, obj?:RequestGetServer):RequestGetServer {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new RequestGetServer()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static startRequestGetServer(builder:flatbuffers.Builder) {
  builder.startObject(0);
}

static endRequestGetServer(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createRequestGetServer(builder:flatbuffers.Builder):flatbuffers.Offset {
  RequestGetServer.startRequestGetServer(builder);
  return RequestGetServer.endRequestGetServer(builder);
}

unpack(): RequestGetServerT {
  return new RequestGetServerT();
}


unpackTo(_o: RequestGetServerT): void {}
}

export class RequestGetServerT implements flatbuffers.IGeneratedObject {
constructor(){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  return RequestGetServer.createRequestGetServer(builder);
}
}
