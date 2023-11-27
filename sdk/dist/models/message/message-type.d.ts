import { ClientUpdateEvent } from '../message/client-update-event.js';
import { InitClientEvent } from '../message/init-client-event.js';
import { InitStateEvent } from '../message/init-state-event.js';
import { NoOpEvent } from '../message/no-op-event.js';
import { RequestServerStatEvent } from '../message/request-server-stat-event.js';
import { ServerStatEvent } from '../message/server-stat-event.js';
export declare enum MessageType {
  NONE = 0,
  NoOpEvent = 1,
  InitClientEvent = 2,
  InitStateEvent = 3,
  ClientUpdateEvent = 4,
  ServerStatEvent = 5,
  RequestServerStatEvent = 6,
}
export declare function unionToMessageType(
  type: MessageType,
  accessor: (
    obj: ClientUpdateEvent | InitClientEvent | InitStateEvent | NoOpEvent | RequestServerStatEvent | ServerStatEvent
  ) =>
    | ClientUpdateEvent
    | InitClientEvent
    | InitStateEvent
    | NoOpEvent
    | RequestServerStatEvent
    | ServerStatEvent
    | null
): ClientUpdateEvent | InitClientEvent | InitStateEvent | NoOpEvent | RequestServerStatEvent | ServerStatEvent | null;
export declare function unionListToMessageType(
  type: MessageType,
  accessor: (
    index: number,
    obj: ClientUpdateEvent | InitClientEvent | InitStateEvent | NoOpEvent | RequestServerStatEvent | ServerStatEvent
  ) =>
    | ClientUpdateEvent
    | InitClientEvent
    | InitStateEvent
    | NoOpEvent
    | RequestServerStatEvent
    | ServerStatEvent
    | null,
  index: number
): ClientUpdateEvent | InitClientEvent | InitStateEvent | NoOpEvent | RequestServerStatEvent | ServerStatEvent | null;
