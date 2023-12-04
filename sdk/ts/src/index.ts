import EventEmitter from 'eventemitter3';
import WebSocket from 'isomorphic-ws';

type DynamicEvents = {
  server_info: { connectedClients: number };
  energy_changed: { current: number; capacity: number };
};

type StaticEvents = {
  connect: void;
  message: string;
  disconnect: { code: number; reason: string };
  error: WebSocket.ErrorEvent;
};

type Events = DynamicEvents & StaticEvents;

/**
 * Represents the SpaceTradersRT class that extends EventEmitter.
 * This class provides functionality for connecting to a WebSocket server and sending/receiving messages.
 */
export class SpaceTradersRT extends EventEmitter {
  private url: string;
  private ws: WebSocket | null;

  private rpcId: number = 0;

  private rpcCallbacks: { [id: number]: (payload: unknown) => void } = {};

  /**
   * Constructs a new instance of the SpaceTradersRT class.
   * Initializes the URL and WebSocket properties.
   */
  constructor() {
    super();
    this.url = 'ws://127.0.0.1:8081/ws';
    this.ws = null;
  }

  /**
   * Emits an event with an optional payload.
   * @param event - The event to emit.
   * @param payload - The payload associated with the event.
   * @returns A boolean indicating if the event was emitted successfully.
   */
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  public emit<K extends keyof Events>(event: K | string | symbol, payload?: any): boolean {
    return super.emit(event, payload);
  }

  /**
   * Registers an event listener for the specified event.
   *
   * @param event - The event to listen for.
   * @param listener - The listener function to be called when the event is emitted.
   * @returns The current instance of the SpaceTradersRT class.
   */
  public on<K extends keyof Events>(
    event: K | string | symbol,
    listener: (payload: Events[K]) => void
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

      // The binary data is currently just JSON
      const decoded = JSON.parse(data.toString('utf8'));

      if ('error' in decoded) {
        console.log('received error:', decoded);
        return;
      }

      if ('event' in decoded) {
        // TODO: validate the event
        this.emit(decoded.event, decoded.payload);
        return;
      }

      if('id' in decoded && 'output' in decoded) {
        const rpcCallback = this.rpcCallbacks[decoded.id];

        if(!rpcCallback) {
          console.warn('received rpc response with no callback:', decoded);
          return;
        }

        rpcCallback(decoded.output);
      }

      // const buffer = new flatbuffers.ByteBuffer(data as Uint8Array);
      // const message = Message.getRootAsMessage(buffer).unpack();

      // if (message.message) {
      //   this.emit(MessageType[message.messageType], message.message);
      // }
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
  public async getServerStats() {
    // const builder = new flatbuffers.Builder(1);
    // const event = RequestGetServer.createRequestGetServer(builder);
    // const message = Message.createMessage(builder, MessageType.RequestGetServer, event);
    // builder.finish(message);
    // this.send(builder.asUint8Array());
    const id = this.rpcId++;

    const promise = new Promise((resolve) => {
      this.rpcCallbacks[id] = (output: unknown) => {
        // TODO(trevin): Validate the response

        resolve(output);
      };
    });

    this.send(Buffer.from(JSON.stringify({ id, command: 'get_server_info' })));

    return promise;
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

export default SpaceTradersRT;
