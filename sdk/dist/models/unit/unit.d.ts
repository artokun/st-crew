import * as flatbuffers from 'flatbuffers';
import { Vec2, Vec2T } from '../shared/vec2.js';
import { UnitState } from '../unit/unit-state.js';
export declare class Unit implements flatbuffers.IUnpackableObject<UnitT> {
  bb: flatbuffers.ByteBuffer | null;
  bb_pos: number;
  __init(i: number, bb: flatbuffers.ByteBuffer): Unit;
  static getRootAsUnit(bb: flatbuffers.ByteBuffer, obj?: Unit): Unit;
  static getSizePrefixedRootAsUnit(bb: flatbuffers.ByteBuffer, obj?: Unit): Unit;
  id(): number;
  name(): string | null;
  name(optionalEncoding: flatbuffers.Encoding): string | Uint8Array | null;
  controlledBy(): string | null;
  controlledBy(optionalEncoding: flatbuffers.Encoding): string | Uint8Array | null;
  state(): UnitState;
  position(obj?: Vec2): Vec2 | null;
  static startUnit(builder: flatbuffers.Builder): void;
  static addId(builder: flatbuffers.Builder, id: number): void;
  static addName(builder: flatbuffers.Builder, nameOffset: flatbuffers.Offset): void;
  static addControlledBy(builder: flatbuffers.Builder, controlledByOffset: flatbuffers.Offset): void;
  static addState(builder: flatbuffers.Builder, state: UnitState): void;
  static addPosition(builder: flatbuffers.Builder, positionOffset: flatbuffers.Offset): void;
  static endUnit(builder: flatbuffers.Builder): flatbuffers.Offset;
  static finishUnitBuffer(builder: flatbuffers.Builder, offset: flatbuffers.Offset): void;
  static finishSizePrefixedUnitBuffer(builder: flatbuffers.Builder, offset: flatbuffers.Offset): void;
  unpack(): UnitT;
  unpackTo(_o: UnitT): void;
}
export declare class UnitT implements flatbuffers.IGeneratedObject {
  id: number;
  name: string | Uint8Array | null;
  controlledBy: string | Uint8Array | null;
  state: UnitState;
  position: Vec2T | null;
  constructor(
    id?: number,
    name?: string | Uint8Array | null,
    controlledBy?: string | Uint8Array | null,
    state?: UnitState,
    position?: Vec2T | null
  );
  pack(builder: flatbuffers.Builder): flatbuffers.Offset;
}
