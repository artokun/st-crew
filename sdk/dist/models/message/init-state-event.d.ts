import * as flatbuffers from 'flatbuffers';
import { Unit, UnitT } from '../unit/unit.js';
export declare class InitStateEvent implements flatbuffers.IUnpackableObject<InitStateEventT> {
  bb: flatbuffers.ByteBuffer | null;
  bb_pos: number;
  __init(i: number, bb: flatbuffers.ByteBuffer): InitStateEvent;
  static getRootAsInitStateEvent(bb: flatbuffers.ByteBuffer, obj?: InitStateEvent): InitStateEvent;
  static getSizePrefixedRootAsInitStateEvent(bb: flatbuffers.ByteBuffer, obj?: InitStateEvent): InitStateEvent;
  units(index: number, obj?: Unit): Unit | null;
  unitsLength(): number;
  static startInitStateEvent(builder: flatbuffers.Builder): void;
  static addUnits(builder: flatbuffers.Builder, unitsOffset: flatbuffers.Offset): void;
  static createUnitsVector(builder: flatbuffers.Builder, data: flatbuffers.Offset[]): flatbuffers.Offset;
  static startUnitsVector(builder: flatbuffers.Builder, numElems: number): void;
  static endInitStateEvent(builder: flatbuffers.Builder): flatbuffers.Offset;
  static createInitStateEvent(builder: flatbuffers.Builder, unitsOffset: flatbuffers.Offset): flatbuffers.Offset;
  unpack(): InitStateEventT;
  unpackTo(_o: InitStateEventT): void;
}
export declare class InitStateEventT implements flatbuffers.IGeneratedObject {
  units: UnitT[];
  constructor(units?: UnitT[]);
  pack(builder: flatbuffers.Builder): flatbuffers.Offset;
}
