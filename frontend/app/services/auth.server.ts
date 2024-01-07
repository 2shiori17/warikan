import { Authenticator } from "remix-auth";
import { Auth0Strategy } from "remix-auth-auth0";

import { AUTH0_DOMAIN, AUTH0_CLIENT_ID, AUTH0_CLIENT_SECRET, AUTH0_CALLBACK_URL, AUTH0_AUDIENCE } from "~/services/constants.server";
import { sessionStorage } from "~/services/session.server";

export interface AuthUser {
  token: string,
}

export const authenticator = new Authenticator<AuthUser>(sessionStorage);

const auth0Strategy = new Auth0Strategy(
  {
    domain: AUTH0_DOMAIN,
    clientID: AUTH0_CLIENT_ID,
    clientSecret: AUTH0_CLIENT_SECRET,
    callbackURL: AUTH0_CALLBACK_URL,
    audience: AUTH0_AUDIENCE,
  },
  async (data) => {
    console.log(data);
    return { token: data.accessToken }
  }
);

authenticator.use(auth0Strategy);
