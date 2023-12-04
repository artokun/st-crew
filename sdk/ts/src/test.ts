import * as z from "zod";

export const GetServerInfoResponseSchema = z.object({ connectedClients: z.number() });

export type GetServerInfoResponse = z.infer<typeof GetServerInfoResponseSchema>;

interface RpcCommands {
    get_server_info: () => GetServerInfoResponse;
};

type RpcRequest<C extends keyof RpcCommands> =
    RpcCommands[C] extends ((input: infer Input) => unknown) ?
        never extends Input
            ? { id: number; command: C; }
            : { id: number; command: C; input: Input; }
    : never;

type RpcResponse<C extends keyof RpcCommands> = RpcCommands[C] extends ((input: string) => infer Output) ? {
    id: number;
    output: Output;
} : never;

type Test1 = RpcRequest<"get_server_info">;
type Test2 = RpcResponse<"get_server_info">;
