// automatically generated by the FlatBuffers compiler, do not modify

import { GetClient, GetClientT } from '../message/get-client.js';
import { GetServer, GetServerT } from '../message/get-server.js';
import { NoOp, NoOpT } from '../message/no-op.js';
import { RequestGetClient, RequestGetClientT } from '../message/request-get-client.js';
import { RequestGetServer, RequestGetServerT } from '../message/request-get-server.js';


export enum MessageType {
  NONE = 0,
  NoOp = 1,
  GetServer = 2,
  GetClient = 3,
  RequestGetServer = 4,
  RequestGetClient = 5
}

export function unionToMessageType(
  type: MessageType,
  accessor: (obj:GetClient|GetServer|NoOp|RequestGetClient|RequestGetServer) => GetClient|GetServer|NoOp|RequestGetClient|RequestGetServer|null
): GetClient|GetServer|NoOp|RequestGetClient|RequestGetServer|null {
  switch(MessageType[type]) {
    case 'NONE': return null; 
    case 'NoOp': return accessor(new NoOp())! as NoOp;
    case 'GetServer': return accessor(new GetServer())! as GetServer;
    case 'GetClient': return accessor(new GetClient())! as GetClient;
    case 'RequestGetServer': return accessor(new RequestGetServer())! as RequestGetServer;
    case 'RequestGetClient': return accessor(new RequestGetClient())! as RequestGetClient;
    default: return null;
  }
}

export function unionListToMessageType(
  type: MessageType, 
  accessor: (index: number, obj:GetClient|GetServer|NoOp|RequestGetClient|RequestGetServer) => GetClient|GetServer|NoOp|RequestGetClient|RequestGetServer|null, 
  index: number
): GetClient|GetServer|NoOp|RequestGetClient|RequestGetServer|null {
  switch(MessageType[type]) {
    case 'NONE': return null; 
    case 'NoOp': return accessor(index, new NoOp())! as NoOp;
    case 'GetServer': return accessor(index, new GetServer())! as GetServer;
    case 'GetClient': return accessor(index, new GetClient())! as GetClient;
    case 'RequestGetServer': return accessor(index, new RequestGetServer())! as RequestGetServer;
    case 'RequestGetClient': return accessor(index, new RequestGetClient())! as RequestGetClient;
    default: return null;
  }
}
