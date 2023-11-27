import EventEmitter from 'eventemitter3';
import WebSocket from 'isomorphic-ws';
import * as flatbuffers from 'flatbuffers';
import { Message, MessageType, RequestGetServer, GetServerT } from './generated/message';

type DynamicPayloads = {
  GetServer: GetServerT;
};

type StaticPayloads = {
  connect: void;
  message: string;
  disconnect: { code: number; reason: string };
  error: WebSocket.ErrorEvent;
};

type MessagePayloads = DynamicPayloads & StaticPayloads;

/**
 * Represents the SpaceTradersRT class that extends EventEmitter.
 * This class provides functionality for connecting to a WebSocket server and sending/receiving messages.
 */
export class SpaceTradersRT extends EventEmitter {
  private url: string;
  private ws: WebSocket | null;

  /**
   * Constructs a new instance of the SpaceTradersRT class.
   * Initializes the URL and WebSocket properties.
   */
  constructor() {
    super();
    this.url = 'ws://localhost:8080';
    this.ws = null;
  }

  /**
   * Emits an event with an optional payload.
   * @param event - The event to emit.
   * @param payload - The payload associated with the event.
   * @returns A boolean indicating if the event was emitted successfully.
   */
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  public emit<K extends keyof typeof MessageType>(event: K | string | symbol, payload?: any): boolean {
    return super.emit(event, payload);
  }

  /**
   * Registers an event listener for the specified event.
   * @param event - The event to listen for.
   * @param listener - The listener function to be called when the event is emitted.
   * @returns The current instance of the SpaceTradersRT class.
   */
  public on<K extends keyof typeof MessageType>(
    event: K | string | symbol,
    // @ts-expect-error - this is a hack to get the payload type to be strongly typed
    listener: (payload: MessagePayloads[K]) => void
  ): this {
    return super.on(event, listener);
  }

  /**
   * Connects to the WebSocket server.
   * @returns A promise that resolves when the connection is established.
   */
  async connect() {
    this.ws = new WebSocket(this.url);

    this.ws.onopen = () => {
      this.emit('connect');
    };

    this.ws.onmessage = ({ data }) => {
      if (!(typeof data === 'object')) {
        this.emit('message', data);
        return;
      }

      const buffer = new flatbuffers.ByteBuffer(data as Uint8Array);
      const message = Message.getRootAsMessage(buffer).unpack();

      if (message.message) {
        this.emit(MessageType[message.messageType], message.message);
      }
    };

    this.ws.onerror = (error) => {
      this.emit('error', error);
    };

    this.ws.onclose = (event) => {
      this.emit('disconnect', { code: event.code, reason: event.reason });

      // Reconnect logic
      setTimeout(() => this.connect(), 1000);
    };

    return new Promise((resolve) => {
      this.once('connect', () => {
        resolve(true);
      });
    });
  }

  /**
   * Sends a request to get the server statistics.
   */
  public getServerStats() {
    const builder = new flatbuffers.Builder(1);
    const event = RequestGetServer.createRequestGetServer(builder);
    const message = Message.createMessage(builder, MessageType.RequestGetServer, event);
    builder.finish(message);
    this.send(builder.asUint8Array());
  }

  /**
   * Sends data to the WebSocket server.
   * @param data - The data to send.
   */
  private send(data: Uint8Array) {
    this.ws?.send(data);
  }

  /**
   * Disconnects from the WebSocket server.
   */
  private disconnect() {
    this.ws?.close();
  }
}

// reexport the MessageType enum so that it can be used in the client with string literals as the keys
export const MessageTypes = Array.from(Object.keys(MessageType)).reduce((acc, key) => {
  // @ts-expect-error - this is a hack to get the payload type to be strongly typed
  acc[key] = key;
  return acc;
}, {} as { [key in keyof typeof MessageType]: key });

export default SpaceTradersRT;
