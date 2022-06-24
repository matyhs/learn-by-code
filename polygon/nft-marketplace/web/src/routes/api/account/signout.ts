import { serialize } from "cookie";

export const get = (input: {
                         request: Request
                    }): Response => 
{
    const cookie = input.request.headers.get("cookie");
    
    if (cookie == null) {
        return new Response("Error signing out user", { "status": 500 });
    }

    const init = { 
        "status": 200, 
        "headers": {
            "Set-Cookie": serialize("token", "", {
                path: "/",
                expires: new Date(0)
            })
        }
    };

     return new Response("Sign out successfully", init);
}