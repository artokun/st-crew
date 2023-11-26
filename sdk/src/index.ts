import EventEmitter from "eventemitter3";
import WebSocket from "isomorphic-ws";
import * as flatbuffers from "flatbuffers";
import {
  Message,
  MessageType,
  RequestServerStatEvent,
  ServerStatEventT,
} from "./models/message";

type DynamicPayloads = {
  ServerStatEvent: ServerStatEventT;
};

type StaticPayloads = {
  connect: void;
  disconnect: { code: number; reason: string };
  error: WebSocket.ErrorEvent;
};

type MessagePayloads = DynamicPayloads & StaticPayloads;

export class SpaceTradersRT extends EventEmitter {
  private url: string;
  private ws: WebSocket | null;

  constructor() {
    super();
    this.url = "ws://localhost:8080";
    this.ws = null;
  }

  public emit<K extends keyof typeof MessageType>(
    event: K | string | symbol,
    payload?: any
  ): boolean {
    return super.emit(event, payload);
  }

  public on<K extends keyof typeof MessageType>(
    event: K | string | symbol,
    // @ts-expect-error
    listener: (payload: MessagePayloads[K]) => void
  ): this {
    // Now the event type and payload type are strongly coupled
    return super.on(event, listener);
  }

  async connect() {
    this.ws = new WebSocket(this.url);

    this.ws.onopen = () => {
      this.emit("connect");
    };

    this.ws.onmessage = ({ data }) => {
      const buffer = new flatbuffers.ByteBuffer(data as Uint8Array);
      const message = Message.getRootAsMessage(buffer).unpack();

      if (message.message) {
        this.emit(MessageType[message.messageType], message.message);
      }
    };

    this.ws.onerror = (error) => {
      this.emit("error", error);
    };

    this.ws.onclose = (event) => {
      this.emit("disconnect", { code: event.code, reason: event.reason });

      // Reconnect logic
      setTimeout(() => this.connect(), 1000);
    };

    return new Promise((resolve) => {
      this.once("connect", () => {
        resolve(true);
      });
    });
  }

  public getServerStats() {
    const builder = new flatbuffers.Builder(1);
    const event = RequestServerStatEvent.createRequestServerStatEvent(builder);
    const message = Message.createMessage(
      builder,
      MessageType.RequestServerStatEvent,
      event
    );
    builder.finish(message);
    this.send(builder.asUint8Array());
  }

  private send(data: Uint8Array) {
    this.ws?.send(data);
  }

  private disconnect() {
    this.ws?.close();
  }
}

export default SpaceTradersRT;
