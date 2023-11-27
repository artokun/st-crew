// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { Client, ClientT } from '../client/client.js';


export class GetClient implements flatbuffers.IUnpackableObject<GetClientT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):GetClient {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsGetClient(bb:flatbuffers.ByteBuffer, obj?:GetClient):GetClient {
  return (obj || new GetClient()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsGetClient(bb:flatbuffers.ByteBuffer, obj?:GetClient):GetClient {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new GetClient()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

client(obj?:Client):Client|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? (obj || new Client()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!) : null;
}

static startGetClient(builder:flatbuffers.Builder) {
  builder.startObject(1);
}

static addClient(builder:flatbuffers.Builder, clientOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, clientOffset, 0);
}

static endGetClient(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createGetClient(builder:flatbuffers.Builder, clientOffset:flatbuffers.Offset):flatbuffers.Offset {
  GetClient.startGetClient(builder);
  GetClient.addClient(builder, clientOffset);
  return GetClient.endGetClient(builder);
}

unpack(): GetClientT {
  return new GetClientT(
    (this.client() !== null ? this.client()!.unpack() : null)
  );
}


unpackTo(_o: GetClientT): void {
  _o.client = (this.client() !== null ? this.client()!.unpack() : null);
}
}

export class GetClientT implements flatbuffers.IGeneratedObject {
constructor(
  public client: ClientT|null = null
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const client = (this.client !== null ? this.client!.pack(builder) : 0);

  return GetClient.createGetClient(builder,
    client
  );
}
}
