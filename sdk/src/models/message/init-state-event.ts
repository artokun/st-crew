// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { Unit, UnitT } from '../unit/unit.js';


export class InitStateEvent implements flatbuffers.IUnpackableObject<InitStateEventT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):InitStateEvent {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsInitStateEvent(bb:flatbuffers.ByteBuffer, obj?:InitStateEvent):InitStateEvent {
  return (obj || new InitStateEvent()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsInitStateEvent(bb:flatbuffers.ByteBuffer, obj?:InitStateEvent):InitStateEvent {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new InitStateEvent()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

units(index: number, obj?:Unit):Unit|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? (obj || new Unit()).__init(this.bb!.__indirect(this.bb!.__vector(this.bb_pos + offset) + index * 4), this.bb!) : null;
}

unitsLength():number {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

static startInitStateEvent(builder:flatbuffers.Builder) {
  builder.startObject(1);
}

static addUnits(builder:flatbuffers.Builder, unitsOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, unitsOffset, 0);
}

static createUnitsVector(builder:flatbuffers.Builder, data:flatbuffers.Offset[]):flatbuffers.Offset {
  builder.startVector(4, data.length, 4);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addOffset(data[i]!);
  }
  return builder.endVector();
}

static startUnitsVector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(4, numElems, 4);
}

static endInitStateEvent(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createInitStateEvent(builder:flatbuffers.Builder, unitsOffset:flatbuffers.Offset):flatbuffers.Offset {
  InitStateEvent.startInitStateEvent(builder);
  InitStateEvent.addUnits(builder, unitsOffset);
  return InitStateEvent.endInitStateEvent(builder);
}

unpack(): InitStateEventT {
  return new InitStateEventT(
    this.bb!.createObjList<Unit, UnitT>(this.units.bind(this), this.unitsLength())
  );
}


unpackTo(_o: InitStateEventT): void {
  _o.units = this.bb!.createObjList<Unit, UnitT>(this.units.bind(this), this.unitsLength());
}
}

export class InitStateEventT implements flatbuffers.IGeneratedObject {
constructor(
  public units: (UnitT)[] = []
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const units = InitStateEvent.createUnitsVector(builder, builder.createObjectOffsetList(this.units));

  return InitStateEvent.createInitStateEvent(builder,
    units
  );
}
}
