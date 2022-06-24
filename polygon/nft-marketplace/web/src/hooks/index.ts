import type { User } from "$lib/types/class";
import type { RequestEvent, ResolveOptions } from "@sveltejs/kit";
import type { MaybePromise } from "@sveltejs/kit/types/private";
import { parse } from "cookie";
import jwt from "jsonwebtoken";

const { verify } = jwt;

const handle = async (input: {
                                event: RequestEvent;
                                resolve(
                                    event: RequestEvent,
                                    opts?: ResolveOptions
                                ): MaybePromise<Response>    
                            }): Promise<Response> => 
{
    const cookie = input.event.request.headers.get("cookie");
    if (cookie != null)
    {
        const token = parse(cookie).token;
        verify(token, "xyzmktsecret", (err, decoded) => {
            if (err) {
                return new Response();
            }

            if (decoded != undefined) {
                const user = decoded as User;
                input.event.locals.user = user;
            }
        });
    }

    const response = await input.resolve(input.event);
    return response;
}

const getSession = (event: RequestEvent) => {
    return event.locals.user
    ? {
        user: {
            id: event.locals.user.id
        }
    } : {};
}

export { handle, getSession };