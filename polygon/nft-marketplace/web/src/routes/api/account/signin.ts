import { serialize } from "cookie";
import { ethers } from "ethers";
import { SIGN_MESSAGE } from "$lib/types/constants";
import jwt from 'jsonwebtoken';
import { User } from "$lib/types/class";

const { sign } = jwt;
const secret = "xyzmktsecret";

export const post = async (input: {
                         request: Request
                    }): Promise<Response> => {
     const data = await input.request.json();
     const account = data.account;
     const message = data.message;
     
     if (account == null) {
          return new Response("Error signing in user", { "status": 500 });
     }

     const byteMessage = ethers.utils.toUtf8Bytes(SIGN_MESSAGE);
     const address = ethers.utils.verifyMessage(byteMessage, message);

     if (address != account) {
          return new Response("Invalid owner", { "status": 401 });
     }

     const user = new User(account);
     const token = sign(JSON.stringify(user), secret);

     const init = { 
          "status": 200, 
          "headers": {
               "Set-Cookie": serialize("token", token, {
                    path: "/",
                    httpOnly: true,
                    sameSite: "strict",
                    secure: true,
                    maxAge: 60 * 60 * 24,
               })
          }
     };

     return new Response("Sign in successfully", init);
}