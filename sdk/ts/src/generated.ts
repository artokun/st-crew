import * as z from "zod";


export const CommandSchema = z.enum([
    "get_server_info",
    "some_other_command",
]);
export type Command = z.infer<typeof CommandSchema>;

export const GeneratedSchema = z.object({
    "command": CommandSchema,
    "with_stuff": z.union([z.null(), z.string()]).optional(),
});
export type Generated = z.infer<typeof GeneratedSchema>;
