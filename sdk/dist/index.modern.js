import t from 'isomorphic-ws';
var e,
  s,
  i =
    ((e = function (t) {
      var e = Object.prototype.hasOwnProperty,
        s = '~';
      function i() {}
      function n(t, e, s) {
        (this.fn = t), (this.context = e), (this.once = s || !1);
      }
      function r(t, e, i, r, o) {
        if ('function' != typeof i) throw new TypeError('The listener must be a function');
        var a = new n(i, r || t, o),
          h = s ? s + e : e;
        return (
          t._events[h]
            ? t._events[h].fn
              ? (t._events[h] = [t._events[h], a])
              : t._events[h].push(a)
            : ((t._events[h] = a), t._eventsCount++),
          t
        );
      }
      function o(t, e) {
        0 == --t._eventsCount ? (t._events = new i()) : delete t._events[e];
      }
      function a() {
        (this._events = new i()), (this._eventsCount = 0);
      }
      Object.create && ((i.prototype = Object.create(null)), new i().__proto__ || (s = !1)),
        (a.prototype.eventNames = function () {
          var t,
            i,
            n = [];
          if (0 === this._eventsCount) return n;
          for (i in (t = this._events)) e.call(t, i) && n.push(s ? i.slice(1) : i);
          return Object.getOwnPropertySymbols ? n.concat(Object.getOwnPropertySymbols(t)) : n;
        }),
        (a.prototype.listeners = function (t) {
          var e = this._events[s ? s + t : t];
          if (!e) return [];
          if (e.fn) return [e.fn];
          for (var i = 0, n = e.length, r = new Array(n); i < n; i++) r[i] = e[i].fn;
          return r;
        }),
        (a.prototype.listenerCount = function (t) {
          var e = this._events[s ? s + t : t];
          return e ? (e.fn ? 1 : e.length) : 0;
        }),
        (a.prototype.emit = function (t, e, i, n, r, o) {
          var a = s ? s + t : t;
          if (!this._events[a]) return !1;
          var h,
            c,
            l = this._events[a],
            d = arguments.length;
          if (l.fn) {
            switch ((l.once && this.removeListener(t, l.fn, void 0, !0), d)) {
              case 1:
                return l.fn.call(l.context), !0;
              case 2:
                return l.fn.call(l.context, e), !0;
              case 3:
                return l.fn.call(l.context, e, i), !0;
              case 4:
                return l.fn.call(l.context, e, i, n), !0;
              case 5:
                return l.fn.call(l.context, e, i, n, r), !0;
              case 6:
                return l.fn.call(l.context, e, i, n, r, o), !0;
            }
            for (c = 1, h = new Array(d - 1); c < d; c++) h[c - 1] = arguments[c];
            l.fn.apply(l.context, h);
          } else {
            var b,
              u = l.length;
            for (c = 0; c < u; c++)
              switch ((l[c].once && this.removeListener(t, l[c].fn, void 0, !0), d)) {
                case 1:
                  l[c].fn.call(l[c].context);
                  break;
                case 2:
                  l[c].fn.call(l[c].context, e);
                  break;
                case 3:
                  l[c].fn.call(l[c].context, e, i);
                  break;
                case 4:
                  l[c].fn.call(l[c].context, e, i, n);
                  break;
                default:
                  if (!h) for (b = 1, h = new Array(d - 1); b < d; b++) h[b - 1] = arguments[b];
                  l[c].fn.apply(l[c].context, h);
              }
          }
          return !0;
        }),
        (a.prototype.on = function (t, e, s) {
          return r(this, t, e, s, !1);
        }),
        (a.prototype.once = function (t, e, s) {
          return r(this, t, e, s, !0);
        }),
        (a.prototype.removeListener = function (t, e, i, n) {
          var r = s ? s + t : t;
          if (!this._events[r]) return this;
          if (!e) return o(this, r), this;
          var a = this._events[r];
          if (a.fn) a.fn !== e || (n && !a.once) || (i && a.context !== i) || o(this, r);
          else {
            for (var h = 0, c = [], l = a.length; h < l; h++)
              (a[h].fn !== e || (n && !a[h].once) || (i && a[h].context !== i)) && c.push(a[h]);
            c.length ? (this._events[r] = 1 === c.length ? c[0] : c) : o(this, r);
          }
          return this;
        }),
        (a.prototype.removeAllListeners = function (t) {
          var e;
          return (
            t ? this._events[(e = s ? s + t : t)] && o(this, e) : ((this._events = new i()), (this._eventsCount = 0)),
            this
          );
        }),
        (a.prototype.off = a.prototype.removeListener),
        (a.prototype.addListener = a.prototype.on),
        (a.prefixed = s),
        (a.EventEmitter = a),
        (t.exports = a);
    }),
    e((s = { exports: {} })),
    s.exports);
const n = new Int32Array(2),
  r = new Float32Array(n.buffer),
  o = new Float64Array(n.buffer),
  a = 1 === new Uint16Array(new Uint8Array([1, 0]).buffer)[0];
var h, c, l, d;
!(function (t) {
  (t[(t.UTF8_BYTES = 1)] = 'UTF8_BYTES'), (t[(t.UTF16_STRING = 2)] = 'UTF16_STRING');
})(h || (h = {}));
class b {
  constructor(t) {
    (this.bytes_ = t), (this.position_ = 0), (this.text_decoder_ = new TextDecoder());
  }
  static allocate(t) {
    return new b(new Uint8Array(t));
  }
  clear() {
    this.position_ = 0;
  }
  bytes() {
    return this.bytes_;
  }
  position() {
    return this.position_;
  }
  setPosition(t) {
    this.position_ = t;
  }
  capacity() {
    return this.bytes_.length;
  }
  readInt8(t) {
    return (this.readUint8(t) << 24) >> 24;
  }
  readUint8(t) {
    return this.bytes_[t];
  }
  readInt16(t) {
    return (this.readUint16(t) << 16) >> 16;
  }
  readUint16(t) {
    return this.bytes_[t] | (this.bytes_[t + 1] << 8);
  }
  readInt32(t) {
    return this.bytes_[t] | (this.bytes_[t + 1] << 8) | (this.bytes_[t + 2] << 16) | (this.bytes_[t + 3] << 24);
  }
  readUint32(t) {
    return this.readInt32(t) >>> 0;
  }
  readInt64(t) {
    return BigInt.asIntN(64, BigInt(this.readUint32(t)) + (BigInt(this.readUint32(t + 4)) << BigInt(32)));
  }
  readUint64(t) {
    return BigInt.asUintN(64, BigInt(this.readUint32(t)) + (BigInt(this.readUint32(t + 4)) << BigInt(32)));
  }
  readFloat32(t) {
    return (n[0] = this.readInt32(t)), r[0];
  }
  readFloat64(t) {
    return (n[a ? 0 : 1] = this.readInt32(t)), (n[a ? 1 : 0] = this.readInt32(t + 4)), o[0];
  }
  writeInt8(t, e) {
    this.bytes_[t] = e;
  }
  writeUint8(t, e) {
    this.bytes_[t] = e;
  }
  writeInt16(t, e) {
    (this.bytes_[t] = e), (this.bytes_[t + 1] = e >> 8);
  }
  writeUint16(t, e) {
    (this.bytes_[t] = e), (this.bytes_[t + 1] = e >> 8);
  }
  writeInt32(t, e) {
    (this.bytes_[t] = e), (this.bytes_[t + 1] = e >> 8), (this.bytes_[t + 2] = e >> 16), (this.bytes_[t + 3] = e >> 24);
  }
  writeUint32(t, e) {
    (this.bytes_[t] = e), (this.bytes_[t + 1] = e >> 8), (this.bytes_[t + 2] = e >> 16), (this.bytes_[t + 3] = e >> 24);
  }
  writeInt64(t, e) {
    this.writeInt32(t, Number(BigInt.asIntN(32, e))),
      this.writeInt32(t + 4, Number(BigInt.asIntN(32, e >> BigInt(32))));
  }
  writeUint64(t, e) {
    this.writeUint32(t, Number(BigInt.asUintN(32, e))),
      this.writeUint32(t + 4, Number(BigInt.asUintN(32, e >> BigInt(32))));
  }
  writeFloat32(t, e) {
    (r[0] = e), this.writeInt32(t, n[0]);
  }
  writeFloat64(t, e) {
    (o[0] = e), this.writeInt32(t, n[a ? 0 : 1]), this.writeInt32(t + 4, n[a ? 1 : 0]);
  }
  getBufferIdentifier() {
    if (this.bytes_.length < this.position_ + 4 + 4)
      throw new Error('FlatBuffers: ByteBuffer is too short to contain an identifier.');
    let t = '';
    for (let e = 0; e < 4; e++) t += String.fromCharCode(this.readInt8(this.position_ + 4 + e));
    return t;
  }
  __offset(t, e) {
    const s = t - this.readInt32(t);
    return e < this.readInt16(s) ? this.readInt16(s + e) : 0;
  }
  __union(t, e) {
    return (t.bb_pos = e + this.readInt32(e)), (t.bb = this), t;
  }
  __string(t, e) {
    t += this.readInt32(t);
    const s = this.readInt32(t),
      i = this.bytes_.subarray((t += 4), t + s);
    return e === h.UTF8_BYTES ? i : this.text_decoder_.decode(i);
  }
  __union_with_string(t, e) {
    return 'string' == typeof t ? this.__string(e) : this.__union(t, e);
  }
  __indirect(t) {
    return t + this.readInt32(t);
  }
  __vector(t) {
    return t + this.readInt32(t) + 4;
  }
  __vector_len(t) {
    return this.readInt32(t + this.readInt32(t));
  }
  __has_identifier(t) {
    if (4 != t.length) throw new Error('FlatBuffers: file identifier must be length 4');
    for (let e = 0; e < 4; e++) if (t.charCodeAt(e) != this.readInt8(this.position() + 4 + e)) return !1;
    return !0;
  }
  createScalarList(t, e) {
    const s = [];
    for (let i = 0; i < e; ++i) {
      const e = t(i);
      null !== e && s.push(e);
    }
    return s;
  }
  createObjList(t, e) {
    const s = [];
    for (let i = 0; i < e; ++i) {
      const e = t(i);
      null !== e && s.push(e.unpack());
    }
    return s;
  }
}
class u {
  constructor(t) {
    let e;
    (this.minalign = 1),
      (this.vtable = null),
      (this.vtable_in_use = 0),
      (this.isNested = !1),
      (this.object_start = 0),
      (this.vtables = []),
      (this.vector_num_elems = 0),
      (this.force_defaults = !1),
      (this.string_maps = null),
      (this.text_encoder = new TextEncoder()),
      (e = t || 1024),
      (this.bb = b.allocate(e)),
      (this.space = e);
  }
  clear() {
    this.bb.clear(),
      (this.space = this.bb.capacity()),
      (this.minalign = 1),
      (this.vtable = null),
      (this.vtable_in_use = 0),
      (this.isNested = !1),
      (this.object_start = 0),
      (this.vtables = []),
      (this.vector_num_elems = 0),
      (this.force_defaults = !1),
      (this.string_maps = null);
  }
  forceDefaults(t) {
    this.force_defaults = t;
  }
  dataBuffer() {
    return this.bb;
  }
  asUint8Array() {
    return this.bb.bytes().subarray(this.bb.position(), this.bb.position() + this.offset());
  }
  prep(t, e) {
    t > this.minalign && (this.minalign = t);
    const s = (1 + ~(this.bb.capacity() - this.space + e)) & (t - 1);
    for (; this.space < s + t + e; ) {
      const t = this.bb.capacity();
      (this.bb = u.growByteBuffer(this.bb)), (this.space += this.bb.capacity() - t);
    }
    this.pad(s);
  }
  pad(t) {
    for (let e = 0; e < t; e++) this.bb.writeInt8(--this.space, 0);
  }
  writeInt8(t) {
    this.bb.writeInt8((this.space -= 1), t);
  }
  writeInt16(t) {
    this.bb.writeInt16((this.space -= 2), t);
  }
  writeInt32(t) {
    this.bb.writeInt32((this.space -= 4), t);
  }
  writeInt64(t) {
    this.bb.writeInt64((this.space -= 8), t);
  }
  writeFloat32(t) {
    this.bb.writeFloat32((this.space -= 4), t);
  }
  writeFloat64(t) {
    this.bb.writeFloat64((this.space -= 8), t);
  }
  addInt8(t) {
    this.prep(1, 0), this.writeInt8(t);
  }
  addInt16(t) {
    this.prep(2, 0), this.writeInt16(t);
  }
  addInt32(t) {
    this.prep(4, 0), this.writeInt32(t);
  }
  addInt64(t) {
    this.prep(8, 0), this.writeInt64(t);
  }
  addFloat32(t) {
    this.prep(4, 0), this.writeFloat32(t);
  }
  addFloat64(t) {
    this.prep(8, 0), this.writeFloat64(t);
  }
  addFieldInt8(t, e, s) {
    (this.force_defaults || e != s) && (this.addInt8(e), this.slot(t));
  }
  addFieldInt16(t, e, s) {
    (this.force_defaults || e != s) && (this.addInt16(e), this.slot(t));
  }
  addFieldInt32(t, e, s) {
    (this.force_defaults || e != s) && (this.addInt32(e), this.slot(t));
  }
  addFieldInt64(t, e, s) {
    (this.force_defaults || e !== s) && (this.addInt64(e), this.slot(t));
  }
  addFieldFloat32(t, e, s) {
    (this.force_defaults || e != s) && (this.addFloat32(e), this.slot(t));
  }
  addFieldFloat64(t, e, s) {
    (this.force_defaults || e != s) && (this.addFloat64(e), this.slot(t));
  }
  addFieldOffset(t, e, s) {
    (this.force_defaults || e != s) && (this.addOffset(e), this.slot(t));
  }
  addFieldStruct(t, e, s) {
    e != s && (this.nested(e), this.slot(t));
  }
  nested(t) {
    if (t != this.offset()) throw new TypeError('FlatBuffers: struct must be serialized inline.');
  }
  notNested() {
    if (this.isNested) throw new TypeError('FlatBuffers: object serialization must not be nested.');
  }
  slot(t) {
    null !== this.vtable && (this.vtable[t] = this.offset());
  }
  offset() {
    return this.bb.capacity() - this.space;
  }
  static growByteBuffer(t) {
    const e = t.capacity();
    if (3221225472 & e) throw new Error('FlatBuffers: cannot grow buffer beyond 2 gigabytes.');
    const s = e << 1,
      i = b.allocate(s);
    return i.setPosition(s - e), i.bytes().set(t.bytes(), s - e), i;
  }
  addOffset(t) {
    this.prep(4, 0), this.writeInt32(this.offset() - t + 4);
  }
  startObject(t) {
    this.notNested(), null == this.vtable && (this.vtable = []), (this.vtable_in_use = t);
    for (let e = 0; e < t; e++) this.vtable[e] = 0;
    (this.isNested = !0), (this.object_start = this.offset());
  }
  endObject() {
    if (null == this.vtable || !this.isNested) throw new Error('FlatBuffers: endObject called without startObject');
    this.addInt32(0);
    const t = this.offset();
    let e = this.vtable_in_use - 1;
    for (; e >= 0 && 0 == this.vtable[e]; e--);
    const s = e + 1;
    for (; e >= 0; e--) this.addInt16(0 != this.vtable[e] ? t - this.vtable[e] : 0);
    this.addInt16(t - this.object_start);
    const i = 2 * (s + 2);
    this.addInt16(i);
    let n = 0;
    const r = this.space;
    t: for (e = 0; e < this.vtables.length; e++) {
      const t = this.bb.capacity() - this.vtables[e];
      if (i == this.bb.readInt16(t)) {
        for (let e = 2; e < i; e += 2) if (this.bb.readInt16(r + e) != this.bb.readInt16(t + e)) continue t;
        n = this.vtables[e];
        break;
      }
    }
    return (
      n
        ? ((this.space = this.bb.capacity() - t), this.bb.writeInt32(this.space, n - t))
        : (this.vtables.push(this.offset()), this.bb.writeInt32(this.bb.capacity() - t, this.offset() - t)),
      (this.isNested = !1),
      t
    );
  }
  finish(t, e, s) {
    const i = s ? 4 : 0;
    if (e) {
      const t = e;
      if ((this.prep(this.minalign, 8 + i), 4 != t.length))
        throw new TypeError('FlatBuffers: file identifier must be length 4');
      for (let e = 3; e >= 0; e--) this.writeInt8(t.charCodeAt(e));
    }
    this.prep(this.minalign, 4 + i),
      this.addOffset(t),
      i && this.addInt32(this.bb.capacity() - this.space),
      this.bb.setPosition(this.space);
  }
  finishSizePrefixed(t, e) {
    this.finish(t, e, !0);
  }
  requiredField(t, e) {
    const s = this.bb.capacity() - t,
      i = s - this.bb.readInt32(s);
    if (!(e < this.bb.readInt16(i) && 0 != this.bb.readInt16(i + e)))
      throw new TypeError('FlatBuffers: field ' + e + ' must be set');
  }
  startVector(t, e, s) {
    this.notNested(), (this.vector_num_elems = e), this.prep(4, t * e), this.prep(s, t * e);
  }
  endVector() {
    return this.writeInt32(this.vector_num_elems), this.offset();
  }
  createSharedString(t) {
    if (!t) return 0;
    if ((this.string_maps || (this.string_maps = new Map()), this.string_maps.has(t))) return this.string_maps.get(t);
    const e = this.createString(t);
    return this.string_maps.set(t, e), e;
  }
  createString(t) {
    if (null == t) return 0;
    let e;
    (e = t instanceof Uint8Array ? t : this.text_encoder.encode(t)),
      this.addInt8(0),
      this.startVector(1, e.length, 1),
      this.bb.setPosition((this.space -= e.length));
    for (let t = 0, s = this.space, i = this.bb.bytes(); t < e.length; t++) i[s++] = e[t];
    return this.endVector();
  }
  createObjectOffset(t) {
    return null === t ? 0 : 'string' == typeof t ? this.createString(t) : t.pack(this);
  }
  createObjectOffsetList(t) {
    const e = [];
    for (let s = 0; s < t.length; ++s) {
      const i = t[s];
      if (null === i) throw new TypeError('FlatBuffers: Argument for createObjectOffsetList cannot contain null.');
      e.push(this.createObjectOffset(i));
    }
    return e;
  }
  createStructOffsetList(t, e) {
    return e(this, t.length), this.createObjectOffsetList(t.slice().reverse()), this.endVector();
  }
}
class p {
  constructor() {
    (this.bb = null), (this.bb_pos = 0);
  }
  __init(t, e) {
    return (this.bb_pos = t), (this.bb = e), this;
  }
  static getRootAsClient(t, e) {
    return (e || new p()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  static getSizePrefixedRootAsClient(t, e) {
    return t.setPosition(t.position() + 4), (e || new p()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  id(t) {
    const e = this.bb.__offset(this.bb_pos, 4);
    return e ? this.bb.__string(this.bb_pos + e, t) : null;
  }
  name(t) {
    const e = this.bb.__offset(this.bb_pos, 6);
    return e ? this.bb.__string(this.bb_pos + e, t) : null;
  }
  static startClient(t) {
    t.startObject(2);
  }
  static addId(t, e) {
    t.addFieldOffset(0, e, 0);
  }
  static addName(t, e) {
    t.addFieldOffset(1, e, 0);
  }
  static endClient(t) {
    return t.endObject();
  }
  static finishClientBuffer(t, e) {
    t.finish(e);
  }
  static finishSizePrefixedClientBuffer(t, e) {
    t.finish(e, void 0, !0);
  }
  static createClient(t, e, s) {
    return p.startClient(t), p.addId(t, e), p.addName(t, s), p.endClient(t);
  }
  unpack() {
    return new f(this.id(), this.name());
  }
  unpackTo(t) {
    (t.id = this.id()), (t.name = this.name());
  }
}
class f {
  constructor(t = null, e = null) {
    (this.id = void 0), (this.name = void 0), (this.id = t), (this.name = e);
  }
  pack(t) {
    const e = null !== this.id ? t.createString(this.id) : 0,
      s = null !== this.name ? t.createString(this.name) : 0;
    return p.createClient(t, e, s);
  }
}
!(function (t) {
  (t[(t.Joined = 0)] = 'Joined'), (t[(t.Left = 1)] = 'Left');
})(c || (c = {}));
class _ {
  constructor() {
    (this.bb = null), (this.bb_pos = 0);
  }
  __init(t, e) {
    return (this.bb_pos = t), (this.bb = e), this;
  }
  static getRootAsClientUpdateEvent(t, e) {
    return (e || new _()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  static getSizePrefixedRootAsClientUpdateEvent(t, e) {
    return t.setPosition(t.position() + 4), (e || new _()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  action() {
    const t = this.bb.__offset(this.bb_pos, 4);
    return t ? this.bb.readInt8(this.bb_pos + t) : c.Joined;
  }
  client(t) {
    const e = this.bb.__offset(this.bb_pos, 6);
    return e ? (t || new p()).__init(this.bb.__indirect(this.bb_pos + e), this.bb) : null;
  }
  static startClientUpdateEvent(t) {
    t.startObject(2);
  }
  static addAction(t, e) {
    t.addFieldInt8(0, e, c.Joined);
  }
  static addClient(t, e) {
    t.addFieldOffset(1, e, 0);
  }
  static endClientUpdateEvent(t) {
    return t.endObject();
  }
  unpack() {
    return new v(this.action(), null !== this.client() ? this.client().unpack() : null);
  }
  unpackTo(t) {
    (t.action = this.action()), (t.client = null !== this.client() ? this.client().unpack() : null);
  }
}
class v {
  constructor(t = c.Joined, e = null) {
    (this.action = void 0), (this.client = void 0), (this.action = t), (this.client = e);
  }
  pack(t) {
    const e = null !== this.client ? this.client.pack(t) : 0;
    return _.startClientUpdateEvent(t), _.addAction(t, this.action), _.addClient(t, e), _.endClientUpdateEvent(t);
  }
}
class I {
  constructor() {
    (this.bb = null), (this.bb_pos = 0);
  }
  __init(t, e) {
    return (this.bb_pos = t), (this.bb = e), this;
  }
  static getRootAsInitClientEvent(t, e) {
    return (e || new I()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  static getSizePrefixedRootAsInitClientEvent(t, e) {
    return t.setPosition(t.position() + 4), (e || new I()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  client(t) {
    const e = this.bb.__offset(this.bb_pos, 4);
    return e ? (t || new p()).__init(this.bb.__indirect(this.bb_pos + e), this.bb) : null;
  }
  static startInitClientEvent(t) {
    t.startObject(1);
  }
  static addClient(t, e) {
    t.addFieldOffset(0, e, 0);
  }
  static endInitClientEvent(t) {
    return t.endObject();
  }
  static createInitClientEvent(t, e) {
    return I.startInitClientEvent(t), I.addClient(t, e), I.endInitClientEvent(t);
  }
  unpack() {
    return new w(null !== this.client() ? this.client().unpack() : null);
  }
  unpackTo(t) {
    t.client = null !== this.client() ? this.client().unpack() : null;
  }
}
class w {
  constructor(t = null) {
    (this.client = void 0), (this.client = t);
  }
  pack(t) {
    const e = null !== this.client ? this.client.pack(t) : 0;
    return I.createInitClientEvent(t, e);
  }
}
class g {
  constructor() {
    (this.bb = null), (this.bb_pos = 0);
  }
  __init(t, e) {
    return (this.bb_pos = t), (this.bb = e), this;
  }
  x() {
    return this.bb.readFloat32(this.bb_pos);
  }
  y() {
    return this.bb.readFloat32(this.bb_pos + 4);
  }
  static sizeOf() {
    return 8;
  }
  static createVec2(t, e, s) {
    return t.prep(4, 8), t.writeFloat32(s), t.writeFloat32(e), t.offset();
  }
  unpack() {
    return new y(this.x(), this.y());
  }
  unpackTo(t) {
    (t.x = this.x()), (t.y = this.y());
  }
}
class y {
  constructor(t = 0, e = 0) {
    (this.x = void 0), (this.y = void 0), (this.x = t), (this.y = e);
  }
  pack(t) {
    return g.createVec2(t, this.x, this.y);
  }
}
!(function (t) {
  (t[(t.Idle = 0)] = 'Idle'), (t[(t.Moving = 1)] = 'Moving');
})(l || (l = {}));
class S {
  constructor() {
    (this.bb = null), (this.bb_pos = 0);
  }
  __init(t, e) {
    return (this.bb_pos = t), (this.bb = e), this;
  }
  static getRootAsUnit(t, e) {
    return (e || new S()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  static getSizePrefixedRootAsUnit(t, e) {
    return t.setPosition(t.position() + 4), (e || new S()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  id() {
    const t = this.bb.__offset(this.bb_pos, 4);
    return t ? this.bb.readUint32(this.bb_pos + t) : 0;
  }
  name(t) {
    const e = this.bb.__offset(this.bb_pos, 6);
    return e ? this.bb.__string(this.bb_pos + e, t) : null;
  }
  controlledBy(t) {
    const e = this.bb.__offset(this.bb_pos, 8);
    return e ? this.bb.__string(this.bb_pos + e, t) : null;
  }
  state() {
    const t = this.bb.__offset(this.bb_pos, 10);
    return t ? this.bb.readInt8(this.bb_pos + t) : l.Idle;
  }
  position(t) {
    const e = this.bb.__offset(this.bb_pos, 12);
    return e ? (t || new g()).__init(this.bb_pos + e, this.bb) : null;
  }
  static startUnit(t) {
    t.startObject(5);
  }
  static addId(t, e) {
    t.addFieldInt32(0, e, 0);
  }
  static addName(t, e) {
    t.addFieldOffset(1, e, 0);
  }
  static addControlledBy(t, e) {
    t.addFieldOffset(2, e, 0);
  }
  static addState(t, e) {
    t.addFieldInt8(3, e, l.Idle);
  }
  static addPosition(t, e) {
    t.addFieldStruct(4, e, 0);
  }
  static endUnit(t) {
    return t.endObject();
  }
  static finishUnitBuffer(t, e) {
    t.finish(e);
  }
  static finishSizePrefixedUnitBuffer(t, e) {
    t.finish(e, void 0, !0);
  }
  unpack() {
    return new m(
      this.id(),
      this.name(),
      this.controlledBy(),
      this.state(),
      null !== this.position() ? this.position().unpack() : null
    );
  }
  unpackTo(t) {
    (t.id = this.id()),
      (t.name = this.name()),
      (t.controlledBy = this.controlledBy()),
      (t.state = this.state()),
      (t.position = null !== this.position() ? this.position().unpack() : null);
  }
}
class m {
  constructor(t = 0, e = null, s = null, i = l.Idle, n = null) {
    (this.id = void 0),
      (this.name = void 0),
      (this.controlledBy = void 0),
      (this.state = void 0),
      (this.position = void 0),
      (this.id = t),
      (this.name = e),
      (this.controlledBy = s),
      (this.state = i),
      (this.position = n);
  }
  pack(t) {
    const e = null !== this.name ? t.createString(this.name) : 0,
      s = null !== this.controlledBy ? t.createString(this.controlledBy) : 0;
    return (
      S.startUnit(t),
      S.addId(t, this.id),
      S.addName(t, e),
      S.addControlledBy(t, s),
      S.addState(t, this.state),
      S.addPosition(t, null !== this.position ? this.position.pack(t) : 0),
      S.endUnit(t)
    );
  }
}
class E {
  constructor() {
    (this.bb = null), (this.bb_pos = 0);
  }
  __init(t, e) {
    return (this.bb_pos = t), (this.bb = e), this;
  }
  static getRootAsInitStateEvent(t, e) {
    return (e || new E()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  static getSizePrefixedRootAsInitStateEvent(t, e) {
    return t.setPosition(t.position() + 4), (e || new E()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  units(t, e) {
    const s = this.bb.__offset(this.bb_pos, 4);
    return s ? (e || new S()).__init(this.bb.__indirect(this.bb.__vector(this.bb_pos + s) + 4 * t), this.bb) : null;
  }
  unitsLength() {
    const t = this.bb.__offset(this.bb_pos, 4);
    return t ? this.bb.__vector_len(this.bb_pos + t) : 0;
  }
  static startInitStateEvent(t) {
    t.startObject(1);
  }
  static addUnits(t, e) {
    t.addFieldOffset(0, e, 0);
  }
  static createUnitsVector(t, e) {
    t.startVector(4, e.length, 4);
    for (let s = e.length - 1; s >= 0; s--) t.addOffset(e[s]);
    return t.endVector();
  }
  static startUnitsVector(t, e) {
    t.startVector(4, e, 4);
  }
  static endInitStateEvent(t) {
    return t.endObject();
  }
  static createInitStateEvent(t, e) {
    return E.startInitStateEvent(t), E.addUnits(t, e), E.endInitStateEvent(t);
  }
  unpack() {
    return new O(this.bb.createObjList(this.units.bind(this), this.unitsLength()));
  }
  unpackTo(t) {
    t.units = this.bb.createObjList(this.units.bind(this), this.unitsLength());
  }
}
class O {
  constructor(t = []) {
    (this.units = void 0), (this.units = t);
  }
  pack(t) {
    const e = E.createUnitsVector(t, t.createObjectOffsetList(this.units));
    return E.createInitStateEvent(t, e);
  }
}
class F {
  constructor() {
    (this.bb = null), (this.bb_pos = 0);
  }
  __init(t, e) {
    return (this.bb_pos = t), (this.bb = e), this;
  }
  static getRootAsNoOpEvent(t, e) {
    return (e || new F()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  static getSizePrefixedRootAsNoOpEvent(t, e) {
    return t.setPosition(t.position() + 4), (e || new F()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  static startNoOpEvent(t) {
    t.startObject(0);
  }
  static endNoOpEvent(t) {
    return t.endObject();
  }
  static createNoOpEvent(t) {
    return F.startNoOpEvent(t), F.endNoOpEvent(t);
  }
  unpack() {
    return new C();
  }
  unpackTo(t) {}
}
class C {
  constructor() {}
  pack(t) {
    return F.createNoOpEvent(t);
  }
}
class U {
  constructor() {
    (this.bb = null), (this.bb_pos = 0);
  }
  __init(t, e) {
    return (this.bb_pos = t), (this.bb = e), this;
  }
  static getRootAsRequestServerStatEvent(t, e) {
    return (e || new U()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  static getSizePrefixedRootAsRequestServerStatEvent(t, e) {
    return t.setPosition(t.position() + 4), (e || new U()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  static startRequestServerStatEvent(t) {
    t.startObject(0);
  }
  static endRequestServerStatEvent(t) {
    return t.endObject();
  }
  static createRequestServerStatEvent(t) {
    return U.startRequestServerStatEvent(t), U.endRequestServerStatEvent(t);
  }
  unpack() {
    return new k();
  }
  unpackTo(t) {}
}
class k {
  constructor() {}
  pack(t) {
    return U.createRequestServerStatEvent(t);
  }
}
class B {
  constructor() {
    (this.bb = null), (this.bb_pos = 0);
  }
  __init(t, e) {
    return (this.bb_pos = t), (this.bb = e), this;
  }
  static getRootAsServerStatEvent(t, e) {
    return (e || new B()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  static getSizePrefixedRootAsServerStatEvent(t, e) {
    return t.setPosition(t.position() + 4), (e || new B()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  clientsConnected() {
    const t = this.bb.__offset(this.bb_pos, 4);
    return t ? this.bb.readUint32(this.bb_pos + t) : 0;
  }
  static startServerStatEvent(t) {
    t.startObject(1);
  }
  static addClientsConnected(t, e) {
    t.addFieldInt32(0, e, 0);
  }
  static endServerStatEvent(t) {
    return t.endObject();
  }
  static createServerStatEvent(t, e) {
    return B.startServerStatEvent(t), B.addClientsConnected(t, e), B.endServerStatEvent(t);
  }
  unpack() {
    return new N(this.clientsConnected());
  }
  unpackTo(t) {
    t.clientsConnected = this.clientsConnected();
  }
}
class N {
  constructor(t = 0) {
    (this.clientsConnected = void 0), (this.clientsConnected = t);
  }
  pack(t) {
    return B.createServerStatEvent(t, this.clientsConnected);
  }
}
function x(t, e) {
  switch (d[t]) {
    case 'NONE':
    default:
      return null;
    case 'NoOpEvent':
      return e(new F());
    case 'InitClientEvent':
      return e(new I());
    case 'InitStateEvent':
      return e(new E());
    case 'ClientUpdateEvent':
      return e(new _());
    case 'ServerStatEvent':
      return e(new B());
    case 'RequestServerStatEvent':
      return e(new U());
  }
}
!(function (t) {
  (t[(t.NONE = 0)] = 'NONE'),
    (t[(t.NoOpEvent = 1)] = 'NoOpEvent'),
    (t[(t.InitClientEvent = 2)] = 'InitClientEvent'),
    (t[(t.InitStateEvent = 3)] = 'InitStateEvent'),
    (t[(t.ClientUpdateEvent = 4)] = 'ClientUpdateEvent'),
    (t[(t.ServerStatEvent = 5)] = 'ServerStatEvent'),
    (t[(t.RequestServerStatEvent = 6)] = 'RequestServerStatEvent');
})(d || (d = {}));
class j {
  constructor() {
    (this.bb = null), (this.bb_pos = 0);
  }
  __init(t, e) {
    return (this.bb_pos = t), (this.bb = e), this;
  }
  static getRootAsMessage(t, e) {
    return (e || new j()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  static getSizePrefixedRootAsMessage(t, e) {
    return t.setPosition(t.position() + 4), (e || new j()).__init(t.readInt32(t.position()) + t.position(), t);
  }
  messageType() {
    const t = this.bb.__offset(this.bb_pos, 4);
    return t ? this.bb.readUint8(this.bb_pos + t) : d.NONE;
  }
  message(t) {
    const e = this.bb.__offset(this.bb_pos, 6);
    return e ? this.bb.__union(t, this.bb_pos + e) : null;
  }
  static startMessage(t) {
    t.startObject(2);
  }
  static addMessageType(t, e) {
    t.addFieldInt8(0, e, d.NONE);
  }
  static addMessage(t, e) {
    t.addFieldOffset(1, e, 0);
  }
  static endMessage(t) {
    return t.endObject();
  }
  static finishMessageBuffer(t, e) {
    t.finish(e);
  }
  static finishSizePrefixedMessageBuffer(t, e) {
    t.finish(e, void 0, !0);
  }
  static createMessage(t, e, s) {
    return j.startMessage(t), j.addMessageType(t, e), j.addMessage(t, s), j.endMessage(t);
  }
  unpack() {
    return new T(
      this.messageType(),
      (() => {
        const t = x(this.messageType(), this.message.bind(this));
        return null === t ? null : t.unpack();
      })()
    );
  }
  unpackTo(t) {
    (t.messageType = this.messageType()),
      (t.message = (() => {
        const t = x(this.messageType(), this.message.bind(this));
        return null === t ? null : t.unpack();
      })());
  }
}
class T {
  constructor(t = d.NONE, e = null) {
    (this.messageType = void 0), (this.message = void 0), (this.messageType = t), (this.message = e);
  }
  pack(t) {
    const e = t.createObjectOffset(this.message);
    return j.createMessage(t, this.messageType, e);
  }
}
class A extends i {
  constructor() {
    super(), (this.url = void 0), (this.ws = void 0), (this.url = 'ws://localhost:8080'), (this.ws = null);
  }
  emit(t, e) {
    return super.emit(t, e);
  }
  on(t, e) {
    return super.on(t, e);
  }
  async connect() {
    return (
      (this.ws = new t(this.url)),
      (this.ws.onopen = () => {
        this.emit('connect');
      }),
      (this.ws.onmessage = ({ data: t }) => {
        const e = new b(t),
          s = j.getRootAsMessage(e).unpack();
        s.message && this.emit(d[s.messageType], s.message);
      }),
      (this.ws.onerror = (t) => {
        this.emit('error', t);
      }),
      (this.ws.onclose = (t) => {
        this.emit('disconnect', { code: t.code, reason: t.reason }), setTimeout(() => this.connect(), 1e3);
      }),
      new Promise((t) => {
        this.once('connect', () => {
          t(!0);
        });
      })
    );
  }
  getServerStats() {
    const t = new u(1),
      e = U.createRequestServerStatEvent(t),
      s = j.createMessage(t, d.RequestServerStatEvent, e);
    t.finish(s), this.send(t.asUint8Array());
  }
  send(t) {
    var e;
    null == (e = this.ws) || e.send(t);
  }
  disconnect() {
    var t;
    null == (t = this.ws) || t.close();
  }
}
const R = Array.from(Object.keys(d)).reduce((t, e) => ((t[e] = e), t), {});
export { R as MessageTypes, A as SpaceTradersRT, A as default };
