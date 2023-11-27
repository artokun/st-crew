!(function (t, e) {
  'object' == typeof exports && 'undefined' != typeof module
    ? e(exports, require('isomorphic-ws'))
    : 'function' == typeof define && define.amd
      ? define(['exports', 'isomorphic-ws'], e)
      : e(((t || self).rtSdk = {}), t.isomorphicWs);
})(this, function (t, e) {
  function n(t) {
    return t && 'object' == typeof t && 'default' in t ? t : { default: t };
  }
  var i = /*#__PURE__*/ n(e);
  function s(t, e) {
    return (
      (s = Object.setPrototypeOf
        ? Object.setPrototypeOf.bind()
        : function (t, e) {
            return (t.__proto__ = e), t;
          }),
      s(t, e)
    );
  }
  var r = (function (t) {
    var e = { exports: {} };
    return (
      (function (t) {
        var e = Object.prototype.hasOwnProperty,
          n = '~';
        function i() {}
        function s(t, e, n) {
          (this.fn = t), (this.context = e), (this.once = n || !1);
        }
        function r(t, e, i, r, o) {
          if ('function' != typeof i) throw new TypeError('The listener must be a function');
          var a = new s(i, r || t, o),
            c = n ? n + e : e;
          return (
            t._events[c]
              ? t._events[c].fn
                ? (t._events[c] = [t._events[c], a])
                : t._events[c].push(a)
              : ((t._events[c] = a), t._eventsCount++),
            t
          );
        }
        function o(t, e) {
          0 == --t._eventsCount ? (t._events = new i()) : delete t._events[e];
        }
        function a() {
          (this._events = new i()), (this._eventsCount = 0);
        }
        Object.create && ((i.prototype = Object.create(null)), new i().__proto__ || (n = !1)),
          (a.prototype.eventNames = function () {
            var t,
              i,
              s = [];
            if (0 === this._eventsCount) return s;
            for (i in (t = this._events)) e.call(t, i) && s.push(n ? i.slice(1) : i);
            return Object.getOwnPropertySymbols ? s.concat(Object.getOwnPropertySymbols(t)) : s;
          }),
          (a.prototype.listeners = function (t) {
            var e = this._events[n ? n + t : t];
            if (!e) return [];
            if (e.fn) return [e.fn];
            for (var i = 0, s = e.length, r = new Array(s); i < s; i++) r[i] = e[i].fn;
            return r;
          }),
          (a.prototype.listenerCount = function (t) {
            var e = this._events[n ? n + t : t];
            return e ? (e.fn ? 1 : e.length) : 0;
          }),
          (a.prototype.emit = function (t, e, i, s, r, o) {
            var a = n ? n + t : t;
            if (!this._events[a]) return !1;
            var c,
              u,
              h = this._events[a],
              f = arguments.length;
            if (h.fn) {
              switch ((h.once && this.removeListener(t, h.fn, void 0, !0), f)) {
                case 1:
                  return h.fn.call(h.context), !0;
                case 2:
                  return h.fn.call(h.context, e), !0;
                case 3:
                  return h.fn.call(h.context, e, i), !0;
                case 4:
                  return h.fn.call(h.context, e, i, s), !0;
                case 5:
                  return h.fn.call(h.context, e, i, s, r), !0;
                case 6:
                  return h.fn.call(h.context, e, i, s, r, o), !0;
              }
              for (u = 1, c = new Array(f - 1); u < f; u++) c[u - 1] = arguments[u];
              h.fn.apply(h.context, c);
            } else {
              var d,
                l = h.length;
              for (u = 0; u < l; u++)
                switch ((h[u].once && this.removeListener(t, h[u].fn, void 0, !0), f)) {
                  case 1:
                    h[u].fn.call(h[u].context);
                    break;
                  case 2:
                    h[u].fn.call(h[u].context, e);
                    break;
                  case 3:
                    h[u].fn.call(h[u].context, e, i);
                    break;
                  case 4:
                    h[u].fn.call(h[u].context, e, i, s);
                    break;
                  default:
                    if (!c) for (d = 1, c = new Array(f - 1); d < f; d++) c[d - 1] = arguments[d];
                    h[u].fn.apply(h[u].context, c);
                }
            }
            return !0;
          }),
          (a.prototype.on = function (t, e, n) {
            return r(this, t, e, n, !1);
          }),
          (a.prototype.once = function (t, e, n) {
            return r(this, t, e, n, !0);
          }),
          (a.prototype.removeListener = function (t, e, i, s) {
            var r = n ? n + t : t;
            if (!this._events[r]) return this;
            if (!e) return o(this, r), this;
            var a = this._events[r];
            if (a.fn) a.fn !== e || (s && !a.once) || (i && a.context !== i) || o(this, r);
            else {
              for (var c = 0, u = [], h = a.length; c < h; c++)
                (a[c].fn !== e || (s && !a[c].once) || (i && a[c].context !== i)) && u.push(a[c]);
              u.length ? (this._events[r] = 1 === u.length ? u[0] : u) : o(this, r);
            }
            return this;
          }),
          (a.prototype.removeAllListeners = function (t) {
            var e;
            return (
              t ? this._events[(e = n ? n + t : t)] && o(this, e) : ((this._events = new i()), (this._eventsCount = 0)),
              this
            );
          }),
          (a.prototype.off = a.prototype.removeListener),
          (a.prototype.addListener = a.prototype.on),
          (a.prefixed = n),
          (a.EventEmitter = a),
          (t.exports = a);
      })(e),
      e.exports
    );
  })();
  const o = new Int32Array(2),
    a = new Float32Array(o.buffer),
    c = new Float64Array(o.buffer),
    u = 1 === new Uint16Array(new Uint8Array([1, 0]).buffer)[0];
  var h;
  !(function (t) {
    (t[(t.UTF8_BYTES = 1)] = 'UTF8_BYTES'), (t[(t.UTF16_STRING = 2)] = 'UTF16_STRING');
  })(h || (h = {}));
  class f {
    constructor(t) {
      (this.bytes_ = t), (this.position_ = 0), (this.text_decoder_ = new TextDecoder());
    }
    static allocate(t) {
      return new f(new Uint8Array(t));
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
      return (o[0] = this.readInt32(t)), a[0];
    }
    readFloat64(t) {
      return (o[u ? 0 : 1] = this.readInt32(t)), (o[u ? 1 : 0] = this.readInt32(t + 4)), c[0];
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
      (this.bytes_[t] = e),
        (this.bytes_[t + 1] = e >> 8),
        (this.bytes_[t + 2] = e >> 16),
        (this.bytes_[t + 3] = e >> 24);
    }
    writeUint32(t, e) {
      (this.bytes_[t] = e),
        (this.bytes_[t + 1] = e >> 8),
        (this.bytes_[t + 2] = e >> 16),
        (this.bytes_[t + 3] = e >> 24);
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
      (a[0] = e), this.writeInt32(t, o[0]);
    }
    writeFloat64(t, e) {
      (c[0] = e), this.writeInt32(t, o[u ? 0 : 1]), this.writeInt32(t + 4, o[u ? 1 : 0]);
    }
    getBufferIdentifier() {
      if (this.bytes_.length < this.position_ + 4 + 4)
        throw new Error('FlatBuffers: ByteBuffer is too short to contain an identifier.');
      let t = '';
      for (let e = 0; e < 4; e++) t += String.fromCharCode(this.readInt8(this.position_ + 4 + e));
      return t;
    }
    __offset(t, e) {
      const n = t - this.readInt32(t);
      return e < this.readInt16(n) ? this.readInt16(n + e) : 0;
    }
    __union(t, e) {
      return (t.bb_pos = e + this.readInt32(e)), (t.bb = this), t;
    }
    __string(t, e) {
      t += this.readInt32(t);
      const n = this.readInt32(t),
        i = this.bytes_.subarray((t += 4), t + n);
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
      const n = [];
      for (let i = 0; i < e; ++i) {
        const e = t(i);
        null !== e && n.push(e);
      }
      return n;
    }
    createObjList(t, e) {
      const n = [];
      for (let i = 0; i < e; ++i) {
        const e = t(i);
        null !== e && n.push(e.unpack());
      }
      return n;
    }
  }
  class d {
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
        (this.bb = f.allocate(e)),
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
      const n = (1 + ~(this.bb.capacity() - this.space + e)) & (t - 1);
      for (; this.space < n + t + e; ) {
        const t = this.bb.capacity();
        (this.bb = d.growByteBuffer(this.bb)), (this.space += this.bb.capacity() - t);
      }
      this.pad(n);
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
    addFieldInt8(t, e, n) {
      (this.force_defaults || e != n) && (this.addInt8(e), this.slot(t));
    }
    addFieldInt16(t, e, n) {
      (this.force_defaults || e != n) && (this.addInt16(e), this.slot(t));
    }
    addFieldInt32(t, e, n) {
      (this.force_defaults || e != n) && (this.addInt32(e), this.slot(t));
    }
    addFieldInt64(t, e, n) {
      (this.force_defaults || e !== n) && (this.addInt64(e), this.slot(t));
    }
    addFieldFloat32(t, e, n) {
      (this.force_defaults || e != n) && (this.addFloat32(e), this.slot(t));
    }
    addFieldFloat64(t, e, n) {
      (this.force_defaults || e != n) && (this.addFloat64(e), this.slot(t));
    }
    addFieldOffset(t, e, n) {
      (this.force_defaults || e != n) && (this.addOffset(e), this.slot(t));
    }
    addFieldStruct(t, e, n) {
      e != n && (this.nested(e), this.slot(t));
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
      const n = e << 1,
        i = f.allocate(n);
      return i.setPosition(n - e), i.bytes().set(t.bytes(), n - e), i;
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
      const n = e + 1;
      for (; e >= 0; e--) this.addInt16(0 != this.vtable[e] ? t - this.vtable[e] : 0);
      this.addInt16(t - this.object_start);
      const i = 2 * (n + 2);
      this.addInt16(i);
      let s = 0;
      const r = this.space;
      t: for (e = 0; e < this.vtables.length; e++) {
        const t = this.bb.capacity() - this.vtables[e];
        if (i == this.bb.readInt16(t)) {
          for (let e = 2; e < i; e += 2) if (this.bb.readInt16(r + e) != this.bb.readInt16(t + e)) continue t;
          s = this.vtables[e];
          break;
        }
      }
      return (
        s
          ? ((this.space = this.bb.capacity() - t), this.bb.writeInt32(this.space, s - t))
          : (this.vtables.push(this.offset()), this.bb.writeInt32(this.bb.capacity() - t, this.offset() - t)),
        (this.isNested = !1),
        t
      );
    }
    finish(t, e, n) {
      const i = n ? 4 : 0;
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
      const n = this.bb.capacity() - t,
        i = n - this.bb.readInt32(n);
      if (!(e < this.bb.readInt16(i) && 0 != this.bb.readInt16(i + e)))
        throw new TypeError('FlatBuffers: field ' + e + ' must be set');
    }
    startVector(t, e, n) {
      this.notNested(), (this.vector_num_elems = e), this.prep(4, t * e), this.prep(n, t * e);
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
      for (let t = 0, n = this.space, i = this.bb.bytes(); t < e.length; t++) i[n++] = e[t];
      return this.endVector();
    }
    createObjectOffset(t) {
      return null === t ? 0 : 'string' == typeof t ? this.createString(t) : t.pack(this);
    }
    createObjectOffsetList(t) {
      const e = [];
      for (let n = 0; n < t.length; ++n) {
        const i = t[n];
        if (null === i) throw new TypeError('FlatBuffers: Argument for createObjectOffsetList cannot contain null.');
        e.push(this.createObjectOffset(i));
      }
      return e;
    }
    createStructOffsetList(t, e) {
      return e(this, t.length), this.createObjectOffsetList(t.slice().reverse()), this.endVector();
    }
  }
  var l,
    b = /*#__PURE__*/ (function () {
      function t() {
        (this.bb = null), (this.bb_pos = 0);
      }
      var e = t.prototype;
      return (
        (e.__init = function (t, e) {
          return (this.bb_pos = t), (this.bb = e), this;
        }),
        (t.getRootAsClient = function (e, n) {
          return (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (t.getSizePrefixedRootAsClient = function (e, n) {
          return e.setPosition(e.position() + 4), (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (e.id = function (t) {
          var e = this.bb.__offset(this.bb_pos, 4);
          return e ? this.bb.__string(this.bb_pos + e, t) : null;
        }),
        (e.name = function (t) {
          var e = this.bb.__offset(this.bb_pos, 6);
          return e ? this.bb.__string(this.bb_pos + e, t) : null;
        }),
        (t.startClient = function (t) {
          t.startObject(2);
        }),
        (t.addId = function (t, e) {
          t.addFieldOffset(0, e, 0);
        }),
        (t.addName = function (t, e) {
          t.addFieldOffset(1, e, 0);
        }),
        (t.endClient = function (t) {
          return t.endObject();
        }),
        (t.finishClientBuffer = function (t, e) {
          t.finish(e);
        }),
        (t.finishSizePrefixedClientBuffer = function (t, e) {
          t.finish(e, void 0, !0);
        }),
        (t.createClient = function (e, n, i) {
          return t.startClient(e), t.addId(e, n), t.addName(e, i), t.endClient(e);
        }),
        (e.unpack = function () {
          return new p(this.id(), this.name());
        }),
        (e.unpackTo = function (t) {
          (t.id = this.id()), (t.name = this.name());
        }),
        t
      );
    })(),
    p = /*#__PURE__*/ (function () {
      function t(t, e) {
        void 0 === t && (t = null),
          void 0 === e && (e = null),
          (this.id = void 0),
          (this.name = void 0),
          (this.id = t),
          (this.name = e);
      }
      return (
        (t.prototype.pack = function (t) {
          var e = null !== this.id ? t.createString(this.id) : 0,
            n = null !== this.name ? t.createString(this.name) : 0;
          return b.createClient(t, e, n);
        }),
        t
      );
    })();
  !(function (t) {
    (t[(t.Joined = 0)] = 'Joined'), (t[(t.Left = 1)] = 'Left');
  })(l || (l = {}));
  var _,
    v = /*#__PURE__*/ (function () {
      function t() {
        (this.bb = null), (this.bb_pos = 0);
      }
      var e = t.prototype;
      return (
        (e.__init = function (t, e) {
          return (this.bb_pos = t), (this.bb = e), this;
        }),
        (t.getRootAsClientUpdateEvent = function (e, n) {
          return (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (t.getSizePrefixedRootAsClientUpdateEvent = function (e, n) {
          return e.setPosition(e.position() + 4), (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (e.action = function () {
          var t = this.bb.__offset(this.bb_pos, 4);
          return t ? this.bb.readInt8(this.bb_pos + t) : l.Joined;
        }),
        (e.client = function (t) {
          var e = this.bb.__offset(this.bb_pos, 6);
          return e ? (t || new b()).__init(this.bb.__indirect(this.bb_pos + e), this.bb) : null;
        }),
        (t.startClientUpdateEvent = function (t) {
          t.startObject(2);
        }),
        (t.addAction = function (t, e) {
          t.addFieldInt8(0, e, l.Joined);
        }),
        (t.addClient = function (t, e) {
          t.addFieldOffset(1, e, 0);
        }),
        (t.endClientUpdateEvent = function (t) {
          return t.endObject();
        }),
        (e.unpack = function () {
          return new y(this.action(), null !== this.client() ? this.client().unpack() : null);
        }),
        (e.unpackTo = function (t) {
          (t.action = this.action()), (t.client = null !== this.client() ? this.client().unpack() : null);
        }),
        t
      );
    })(),
    y = /*#__PURE__*/ (function () {
      function t(t, e) {
        void 0 === t && (t = l.Joined),
          void 0 === e && (e = null),
          (this.action = void 0),
          (this.client = void 0),
          (this.action = t),
          (this.client = e);
      }
      return (
        (t.prototype.pack = function (t) {
          var e = null !== this.client ? this.client.pack(t) : 0;
          return v.startClientUpdateEvent(t), v.addAction(t, this.action), v.addClient(t, e), v.endClientUpdateEvent(t);
        }),
        t
      );
    })(),
    I = /*#__PURE__*/ (function () {
      function t() {
        (this.bb = null), (this.bb_pos = 0);
      }
      var e = t.prototype;
      return (
        (e.__init = function (t, e) {
          return (this.bb_pos = t), (this.bb = e), this;
        }),
        (t.getRootAsInitClientEvent = function (e, n) {
          return (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (t.getSizePrefixedRootAsInitClientEvent = function (e, n) {
          return e.setPosition(e.position() + 4), (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (e.client = function (t) {
          var e = this.bb.__offset(this.bb_pos, 4);
          return e ? (t || new b()).__init(this.bb.__indirect(this.bb_pos + e), this.bb) : null;
        }),
        (t.startInitClientEvent = function (t) {
          t.startObject(1);
        }),
        (t.addClient = function (t, e) {
          t.addFieldOffset(0, e, 0);
        }),
        (t.endInitClientEvent = function (t) {
          return t.endObject();
        }),
        (t.createInitClientEvent = function (e, n) {
          return t.startInitClientEvent(e), t.addClient(e, n), t.endInitClientEvent(e);
        }),
        (e.unpack = function () {
          return new w(null !== this.client() ? this.client().unpack() : null);
        }),
        (e.unpackTo = function (t) {
          t.client = null !== this.client() ? this.client().unpack() : null;
        }),
        t
      );
    })(),
    w = /*#__PURE__*/ (function () {
      function t(t) {
        void 0 === t && (t = null), (this.client = void 0), (this.client = t);
      }
      return (
        (t.prototype.pack = function (t) {
          var e = null !== this.client ? this.client.pack(t) : 0;
          return I.createInitClientEvent(t, e);
        }),
        t
      );
    })(),
    g = /*#__PURE__*/ (function () {
      function t() {
        (this.bb = null), (this.bb_pos = 0);
      }
      var e = t.prototype;
      return (
        (e.__init = function (t, e) {
          return (this.bb_pos = t), (this.bb = e), this;
        }),
        (e.x = function () {
          return this.bb.readFloat32(this.bb_pos);
        }),
        (e.y = function () {
          return this.bb.readFloat32(this.bb_pos + 4);
        }),
        (t.sizeOf = function () {
          return 8;
        }),
        (t.createVec2 = function (t, e, n) {
          return t.prep(4, 8), t.writeFloat32(n), t.writeFloat32(e), t.offset();
        }),
        (e.unpack = function () {
          return new m(this.x(), this.y());
        }),
        (e.unpackTo = function (t) {
          (t.x = this.x()), (t.y = this.y());
        }),
        t
      );
    })(),
    m = /*#__PURE__*/ (function () {
      function t(t, e) {
        void 0 === t && (t = 0),
          void 0 === e && (e = 0),
          (this.x = void 0),
          (this.y = void 0),
          (this.x = t),
          (this.y = e);
      }
      return (
        (t.prototype.pack = function (t) {
          return g.createVec2(t, this.x, this.y);
        }),
        t
      );
    })();
  !(function (t) {
    (t[(t.Idle = 0)] = 'Idle'), (t[(t.Moving = 1)] = 'Moving');
  })(_ || (_ = {}));
  var S,
    E = /*#__PURE__*/ (function () {
      function t() {
        (this.bb = null), (this.bb_pos = 0);
      }
      var e = t.prototype;
      return (
        (e.__init = function (t, e) {
          return (this.bb_pos = t), (this.bb = e), this;
        }),
        (t.getRootAsUnit = function (e, n) {
          return (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (t.getSizePrefixedRootAsUnit = function (e, n) {
          return e.setPosition(e.position() + 4), (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (e.id = function () {
          var t = this.bb.__offset(this.bb_pos, 4);
          return t ? this.bb.readUint32(this.bb_pos + t) : 0;
        }),
        (e.name = function (t) {
          var e = this.bb.__offset(this.bb_pos, 6);
          return e ? this.bb.__string(this.bb_pos + e, t) : null;
        }),
        (e.controlledBy = function (t) {
          var e = this.bb.__offset(this.bb_pos, 8);
          return e ? this.bb.__string(this.bb_pos + e, t) : null;
        }),
        (e.state = function () {
          var t = this.bb.__offset(this.bb_pos, 10);
          return t ? this.bb.readInt8(this.bb_pos + t) : _.Idle;
        }),
        (e.position = function (t) {
          var e = this.bb.__offset(this.bb_pos, 12);
          return e ? (t || new g()).__init(this.bb_pos + e, this.bb) : null;
        }),
        (t.startUnit = function (t) {
          t.startObject(5);
        }),
        (t.addId = function (t, e) {
          t.addFieldInt32(0, e, 0);
        }),
        (t.addName = function (t, e) {
          t.addFieldOffset(1, e, 0);
        }),
        (t.addControlledBy = function (t, e) {
          t.addFieldOffset(2, e, 0);
        }),
        (t.addState = function (t, e) {
          t.addFieldInt8(3, e, _.Idle);
        }),
        (t.addPosition = function (t, e) {
          t.addFieldStruct(4, e, 0);
        }),
        (t.endUnit = function (t) {
          return t.endObject();
        }),
        (t.finishUnitBuffer = function (t, e) {
          t.finish(e);
        }),
        (t.finishSizePrefixedUnitBuffer = function (t, e) {
          t.finish(e, void 0, !0);
        }),
        (e.unpack = function () {
          return new O(
            this.id(),
            this.name(),
            this.controlledBy(),
            this.state(),
            null !== this.position() ? this.position().unpack() : null
          );
        }),
        (e.unpackTo = function (t) {
          (t.id = this.id()),
            (t.name = this.name()),
            (t.controlledBy = this.controlledBy()),
            (t.state = this.state()),
            (t.position = null !== this.position() ? this.position().unpack() : null);
        }),
        t
      );
    })(),
    O = /*#__PURE__*/ (function () {
      function t(t, e, n, i, s) {
        void 0 === t && (t = 0),
          void 0 === e && (e = null),
          void 0 === n && (n = null),
          void 0 === i && (i = _.Idle),
          void 0 === s && (s = null),
          (this.id = void 0),
          (this.name = void 0),
          (this.controlledBy = void 0),
          (this.state = void 0),
          (this.position = void 0),
          (this.id = t),
          (this.name = e),
          (this.controlledBy = n),
          (this.state = i),
          (this.position = s);
      }
      return (
        (t.prototype.pack = function (t) {
          var e = null !== this.name ? t.createString(this.name) : 0,
            n = null !== this.controlledBy ? t.createString(this.controlledBy) : 0;
          return (
            E.startUnit(t),
            E.addId(t, this.id),
            E.addName(t, e),
            E.addControlledBy(t, n),
            E.addState(t, this.state),
            E.addPosition(t, null !== this.position ? this.position.pack(t) : 0),
            E.endUnit(t)
          );
        }),
        t
      );
    })(),
    F = /*#__PURE__*/ (function () {
      function t() {
        (this.bb = null), (this.bb_pos = 0);
      }
      var e = t.prototype;
      return (
        (e.__init = function (t, e) {
          return (this.bb_pos = t), (this.bb = e), this;
        }),
        (t.getRootAsInitStateEvent = function (e, n) {
          return (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (t.getSizePrefixedRootAsInitStateEvent = function (e, n) {
          return e.setPosition(e.position() + 4), (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (e.units = function (t, e) {
          var n = this.bb.__offset(this.bb_pos, 4);
          return n
            ? (e || new E()).__init(this.bb.__indirect(this.bb.__vector(this.bb_pos + n) + 4 * t), this.bb)
            : null;
        }),
        (e.unitsLength = function () {
          var t = this.bb.__offset(this.bb_pos, 4);
          return t ? this.bb.__vector_len(this.bb_pos + t) : 0;
        }),
        (t.startInitStateEvent = function (t) {
          t.startObject(1);
        }),
        (t.addUnits = function (t, e) {
          t.addFieldOffset(0, e, 0);
        }),
        (t.createUnitsVector = function (t, e) {
          t.startVector(4, e.length, 4);
          for (var n = e.length - 1; n >= 0; n--) t.addOffset(e[n]);
          return t.endVector();
        }),
        (t.startUnitsVector = function (t, e) {
          t.startVector(4, e, 4);
        }),
        (t.endInitStateEvent = function (t) {
          return t.endObject();
        }),
        (t.createInitStateEvent = function (e, n) {
          return t.startInitStateEvent(e), t.addUnits(e, n), t.endInitStateEvent(e);
        }),
        (e.unpack = function () {
          return new C(this.bb.createObjList(this.units.bind(this), this.unitsLength()));
        }),
        (e.unpackTo = function (t) {
          t.units = this.bb.createObjList(this.units.bind(this), this.unitsLength());
        }),
        t
      );
    })(),
    C = /*#__PURE__*/ (function () {
      function t(t) {
        void 0 === t && (t = []), (this.units = void 0), (this.units = t);
      }
      return (
        (t.prototype.pack = function (t) {
          var e = F.createUnitsVector(t, t.createObjectOffsetList(this.units));
          return F.createInitStateEvent(t, e);
        }),
        t
      );
    })(),
    U = /*#__PURE__*/ (function () {
      function t() {
        (this.bb = null), (this.bb_pos = 0);
      }
      var e = t.prototype;
      return (
        (e.__init = function (t, e) {
          return (this.bb_pos = t), (this.bb = e), this;
        }),
        (t.getRootAsNoOpEvent = function (e, n) {
          return (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (t.getSizePrefixedRootAsNoOpEvent = function (e, n) {
          return e.setPosition(e.position() + 4), (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (t.startNoOpEvent = function (t) {
          t.startObject(0);
        }),
        (t.endNoOpEvent = function (t) {
          return t.endObject();
        }),
        (t.createNoOpEvent = function (e) {
          return t.startNoOpEvent(e), t.endNoOpEvent(e);
        }),
        (e.unpack = function () {
          return new k();
        }),
        (e.unpackTo = function (t) {}),
        t
      );
    })(),
    k = /*#__PURE__*/ (function () {
      function t() {}
      return (
        (t.prototype.pack = function (t) {
          return U.createNoOpEvent(t);
        }),
        t
      );
    })(),
    B = /*#__PURE__*/ (function () {
      function t() {
        (this.bb = null), (this.bb_pos = 0);
      }
      var e = t.prototype;
      return (
        (e.__init = function (t, e) {
          return (this.bb_pos = t), (this.bb = e), this;
        }),
        (t.getRootAsRequestServerStatEvent = function (e, n) {
          return (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (t.getSizePrefixedRootAsRequestServerStatEvent = function (e, n) {
          return e.setPosition(e.position() + 4), (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (t.startRequestServerStatEvent = function (t) {
          t.startObject(0);
        }),
        (t.endRequestServerStatEvent = function (t) {
          return t.endObject();
        }),
        (t.createRequestServerStatEvent = function (e) {
          return t.startRequestServerStatEvent(e), t.endRequestServerStatEvent(e);
        }),
        (e.unpack = function () {
          return new j();
        }),
        (e.unpackTo = function (t) {}),
        t
      );
    })(),
    j = /*#__PURE__*/ (function () {
      function t() {}
      return (
        (t.prototype.pack = function (t) {
          return B.createRequestServerStatEvent(t);
        }),
        t
      );
    })(),
    N = /*#__PURE__*/ (function () {
      function t() {
        (this.bb = null), (this.bb_pos = 0);
      }
      var e = t.prototype;
      return (
        (e.__init = function (t, e) {
          return (this.bb_pos = t), (this.bb = e), this;
        }),
        (t.getRootAsServerStatEvent = function (e, n) {
          return (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (t.getSizePrefixedRootAsServerStatEvent = function (e, n) {
          return e.setPosition(e.position() + 4), (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (e.clientsConnected = function () {
          var t = this.bb.__offset(this.bb_pos, 4);
          return t ? this.bb.readUint32(this.bb_pos + t) : 0;
        }),
        (t.startServerStatEvent = function (t) {
          t.startObject(1);
        }),
        (t.addClientsConnected = function (t, e) {
          t.addFieldInt32(0, e, 0);
        }),
        (t.endServerStatEvent = function (t) {
          return t.endObject();
        }),
        (t.createServerStatEvent = function (e, n) {
          return t.startServerStatEvent(e), t.addClientsConnected(e, n), t.endServerStatEvent(e);
        }),
        (e.unpack = function () {
          return new x(this.clientsConnected());
        }),
        (e.unpackTo = function (t) {
          t.clientsConnected = this.clientsConnected();
        }),
        t
      );
    })(),
    x = /*#__PURE__*/ (function () {
      function t(t) {
        void 0 === t && (t = 0), (this.clientsConnected = void 0), (this.clientsConnected = t);
      }
      return (
        (t.prototype.pack = function (t) {
          return N.createServerStatEvent(t, this.clientsConnected);
        }),
        t
      );
    })();
  function T(t, e) {
    switch (S[t]) {
      case 'NONE':
      default:
        return null;
      case 'NoOpEvent':
        return e(new U());
      case 'InitClientEvent':
        return e(new I());
      case 'InitStateEvent':
        return e(new F());
      case 'ClientUpdateEvent':
        return e(new v());
      case 'ServerStatEvent':
        return e(new N());
      case 'RequestServerStatEvent':
        return e(new B());
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
  })(S || (S = {}));
  var A = /*#__PURE__*/ (function () {
      function t() {
        (this.bb = null), (this.bb_pos = 0);
      }
      var e = t.prototype;
      return (
        (e.__init = function (t, e) {
          return (this.bb_pos = t), (this.bb = e), this;
        }),
        (t.getRootAsMessage = function (e, n) {
          return (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (t.getSizePrefixedRootAsMessage = function (e, n) {
          return e.setPosition(e.position() + 4), (n || new t()).__init(e.readInt32(e.position()) + e.position(), e);
        }),
        (e.messageType = function () {
          var t = this.bb.__offset(this.bb_pos, 4);
          return t ? this.bb.readUint8(this.bb_pos + t) : S.NONE;
        }),
        (e.message = function (t) {
          var e = this.bb.__offset(this.bb_pos, 6);
          return e ? this.bb.__union(t, this.bb_pos + e) : null;
        }),
        (t.startMessage = function (t) {
          t.startObject(2);
        }),
        (t.addMessageType = function (t, e) {
          t.addFieldInt8(0, e, S.NONE);
        }),
        (t.addMessage = function (t, e) {
          t.addFieldOffset(1, e, 0);
        }),
        (t.endMessage = function (t) {
          return t.endObject();
        }),
        (t.finishMessageBuffer = function (t, e) {
          t.finish(e);
        }),
        (t.finishSizePrefixedMessageBuffer = function (t, e) {
          t.finish(e, void 0, !0);
        }),
        (t.createMessage = function (e, n, i) {
          return t.startMessage(e), t.addMessageType(e, n), t.addMessage(e, i), t.endMessage(e);
        }),
        (e.unpack = function () {
          var t,
            e = this;
          return new P(this.messageType(), null === (t = T(e.messageType(), e.message.bind(e))) ? null : t.unpack());
        }),
        (e.unpackTo = function (t) {
          var e,
            n = this;
          (t.messageType = this.messageType()),
            (t.message = null === (e = T(n.messageType(), n.message.bind(n))) ? null : e.unpack());
        }),
        t
      );
    })(),
    P = /*#__PURE__*/ (function () {
      function t(t, e) {
        void 0 === t && (t = S.NONE),
          void 0 === e && (e = null),
          (this.messageType = void 0),
          (this.message = void 0),
          (this.messageType = t),
          (this.message = e);
      }
      return (
        (t.prototype.pack = function (t) {
          var e = t.createObjectOffset(this.message);
          return A.createMessage(t, this.messageType, e);
        }),
        t
      );
    })(),
    R = /*#__PURE__*/ (function (t) {
      var e, n;
      function r() {
        var e;
        return (
          ((e = t.call(this) || this).url = void 0), (e.ws = void 0), (e.url = 'ws://localhost:8080'), (e.ws = null), e
        );
      }
      (n = t), ((e = r).prototype = Object.create(n.prototype)), (e.prototype.constructor = e), s(e, n);
      var o = r.prototype;
      return (
        (o.emit = function (e, n) {
          return t.prototype.emit.call(this, e, n);
        }),
        (o.on = function (e, n) {
          return t.prototype.on.call(this, e, n);
        }),
        (o.connect = function () {
          try {
            var t = this;
            return (
              (t.ws = new i.default(t.url)),
              (t.ws.onopen = function () {
                t.emit('connect');
              }),
              (t.ws.onmessage = function (e) {
                var n = new f(e.data),
                  i = A.getRootAsMessage(n).unpack();
                i.message && t.emit(S[i.messageType], i.message);
              }),
              (t.ws.onerror = function (e) {
                t.emit('error', e);
              }),
              (t.ws.onclose = function (e) {
                t.emit('disconnect', { code: e.code, reason: e.reason }),
                  setTimeout(function () {
                    return t.connect();
                  }, 1e3);
              }),
              Promise.resolve(
                new Promise(function (e) {
                  t.once('connect', function () {
                    e(!0);
                  });
                })
              )
            );
          } catch (t) {
            return Promise.reject(t);
          }
        }),
        (o.getServerStats = function () {
          var t = new d(1),
            e = B.createRequestServerStatEvent(t),
            n = A.createMessage(t, S.RequestServerStatEvent, e);
          t.finish(n), this.send(t.asUint8Array());
        }),
        (o.send = function (t) {
          var e;
          null == (e = this.ws) || e.send(t);
        }),
        (o.disconnect = function () {
          var t;
          null == (t = this.ws) || t.close();
        }),
        r
      );
    })(r),
    L = Array.from(Object.keys(S)).reduce(function (t, e) {
      return (t[e] = e), t;
    }, {});
  (t.MessageTypes = L), (t.SpaceTradersRT = R), (t.default = R);
});
