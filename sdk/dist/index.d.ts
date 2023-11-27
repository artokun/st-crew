/// <reference types="ws" />
import EventEmitter from 'eventemitter3';
import WebSocket from 'isomorphic-ws';
import { MessageType, ServerStatEventT } from './models/message';
type DynamicPayloads = {
  ServerStatEvent: ServerStatEventT;
};
type StaticPayloads = {
  connect: void;
  disconnect: {
    code: number;
    reason: string;
  };
  error: WebSocket.ErrorEvent;
};
type MessagePayloads = DynamicPayloads & StaticPayloads;
export declare class SpaceTradersRT extends EventEmitter {
  private url;
  private ws;
  constructor();
  emit<K extends keyof typeof MessageType>(event: K | string | symbol, payload?: any): boolean;
  on<K extends keyof typeof MessageType>(
    event: K | string | symbol,
    listener: (payload: MessagePayloads[K]) => void
  ): this;
  connect(): Promise<unknown>;
  getServerStats(): void;
  private send;
  private disconnect;
}
export declare const MessageTypes: {
  [x: number]: number;
  readonly NONE: 'NONE';
  readonly NoOpEvent: 'NoOpEvent';
  readonly InitClientEvent: 'InitClientEvent';
  readonly InitStateEvent: 'InitStateEvent';
  readonly ClientUpdateEvent: 'ClientUpdateEvent';
  readonly ServerStatEvent: 'ServerStatEvent';
  readonly RequestServerStatEvent: 'RequestServerStatEvent';
};
export default SpaceTradersRT;
