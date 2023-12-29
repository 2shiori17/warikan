import { Authenticator } from "remix-auth";
import { Auth0Strategy } from "remix-auth-auth0";

import { User } from "~/lib/data";
import { AUTH0_DOMAIN, AUTH0_CLIENT_ID, AUTH0_CLIENT_SECRET, AUTH0_CALLBACK_URL, AUTH0_AUDIENCE } from "~/services/constants.server";
import { sessionStorage } from "~/services/session.server";

export const authenticator = new Authenticator<User>(sessionStorage);

const auth0Strategy = new Auth0Strategy(
  {
    domain: AUTH0_DOMAIN,
    clientID: AUTH0_CLIENT_ID,
    clientSecret: AUTH0_CLIENT_SECRET,
    callbackURL: AUTH0_CALLBACK_URL,
    audience: AUTH0_AUDIENCE,
  },
  async (data) => {
    // TODO(2shiori17): implement logic
    console.log(data);
    return { id: data.profile.id!, name: data.profile.id! }
  }
);

authenticator.use(auth0Strategy);
