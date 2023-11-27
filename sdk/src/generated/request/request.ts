// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { RequestMOTDEvent, RequestMOTDEventT } from '../request/request-motdevent.js';
import { RequestServerStatEvent, RequestServerStatEventT } from '../request/request-server-stat-event.js';
import { RequestType, unionToRequestType, unionListToRequestType } from '../request/request-type.js';

export class Request implements flatbuffers.IUnpackableObject<RequestT> {
  bb: flatbuffers.ByteBuffer | null = null;
  bb_pos = 0;
  __init(i: number, bb: flatbuffers.ByteBuffer): Request {
    this.bb_pos = i;
    this.bb = bb;
    return this;
  }

  static getRootAsRequest(bb: flatbuffers.ByteBuffer, obj?: Request): Request {
    return (obj || new Request()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
  }

  static getSizePrefixedRootAsRequest(bb: flatbuffers.ByteBuffer, obj?: Request): Request {
    bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
    return (obj || new Request()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
  }

  requestType(): RequestType {
    const offset = this.bb!.__offset(this.bb_pos, 4);
    return offset ? this.bb!.readUint8(this.bb_pos + offset) : RequestType.NONE;
  }

  request<T extends flatbuffers.Table>(obj: any): any | null {
    const offset = this.bb!.__offset(this.bb_pos, 6);
    return offset ? this.bb!.__union(obj, this.bb_pos + offset) : null;
  }

  static startRequest(builder: flatbuffers.Builder) {
    builder.startObject(2);
  }

  static addRequestType(builder: flatbuffers.Builder, requestType: RequestType) {
    builder.addFieldInt8(0, requestType, RequestType.NONE);
  }

  static addRequest(builder: flatbuffers.Builder, requestOffset: flatbuffers.Offset) {
    builder.addFieldOffset(1, requestOffset, 0);
  }

  static endRequest(builder: flatbuffers.Builder): flatbuffers.Offset {
    const offset = builder.endObject();
    return offset;
  }

  static finishRequestBuffer(builder: flatbuffers.Builder, offset: flatbuffers.Offset) {
    builder.finish(offset);
  }

  static finishSizePrefixedRequestBuffer(builder: flatbuffers.Builder, offset: flatbuffers.Offset) {
    builder.finish(offset, undefined, true);
  }

  static createRequest(
    builder: flatbuffers.Builder,
    requestType: RequestType,
    requestOffset: flatbuffers.Offset
  ): flatbuffers.Offset {
    Request.startRequest(builder);
    Request.addRequestType(builder, requestType);
    Request.addRequest(builder, requestOffset);
    return Request.endRequest(builder);
  }

  unpack(): RequestT {
    return new RequestT(
      this.requestType(),
      (() => {
        const temp = unionToRequestType(this.requestType(), this.request.bind(this));
        if (temp === null) {
          return null;
        }
        return temp.unpack();
      })()
    );
  }

  unpackTo(_o: RequestT): void {
    _o.requestType = this.requestType();
    _o.request = (() => {
      const temp = unionToRequestType(this.requestType(), this.request.bind(this));
      if (temp === null) {
        return null;
      }
      return temp.unpack();
    })();
  }
}

export class RequestT implements flatbuffers.IGeneratedObject {
  constructor(
    public requestType: RequestType = RequestType.NONE,
    public request: RequestMOTDEventT | RequestServerStatEventT | null = null
  ) {}

  pack(builder: flatbuffers.Builder): flatbuffers.Offset {
    const request = builder.createObjectOffset(this.request);

    return Request.createRequest(builder, this.requestType, request);
  }
}
