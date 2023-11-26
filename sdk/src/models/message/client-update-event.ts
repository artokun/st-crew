// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { Client, ClientT } from '../client/client.js';
import { ClientAction } from '../client/client-action.js';


export class ClientUpdateEvent implements flatbuffers.IUnpackableObject<ClientUpdateEventT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):ClientUpdateEvent {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsClientUpdateEvent(bb:flatbuffers.ByteBuffer, obj?:ClientUpdateEvent):ClientUpdateEvent {
  return (obj || new ClientUpdateEvent()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsClientUpdateEvent(bb:flatbuffers.ByteBuffer, obj?:ClientUpdateEvent):ClientUpdateEvent {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new ClientUpdateEvent()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

action():ClientAction {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readInt8(this.bb_pos + offset) : ClientAction.Joined;
}

client(obj?:Client):Client|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? (obj || new Client()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!) : null;
}

static startClientUpdateEvent(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addAction(builder:flatbuffers.Builder, action:ClientAction) {
  builder.addFieldInt8(0, action, ClientAction.Joined);
}

static addClient(builder:flatbuffers.Builder, clientOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, clientOffset, 0);
}

static endClientUpdateEvent(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}


unpack(): ClientUpdateEventT {
  return new ClientUpdateEventT(
    this.action(),
    (this.client() !== null ? this.client()!.unpack() : null)
  );
}


unpackTo(_o: ClientUpdateEventT): void {
  _o.action = this.action();
  _o.client = (this.client() !== null ? this.client()!.unpack() : null);
}
}

export class ClientUpdateEventT implements flatbuffers.IGeneratedObject {
constructor(
  public action: ClientAction = ClientAction.Joined,
  public client: ClientT|null = null
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const client = (this.client !== null ? this.client!.pack(builder) : 0);

  ClientUpdateEvent.startClientUpdateEvent(builder);
  ClientUpdateEvent.addAction(builder, this.action);
  ClientUpdateEvent.addClient(builder, client);

  return ClientUpdateEvent.endClientUpdateEvent(builder);
}
}