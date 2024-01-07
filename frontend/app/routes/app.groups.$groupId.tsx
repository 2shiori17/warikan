import type { LoaderFunctionArgs } from "@remix-run/node";
import { json } from "@remix-run/node";
import invariant from "tiny-invariant";
import { GraphQLClient } from 'graphql-request';

import { GetGroupDetailQuery } from "~/lib/data";
import { API_URL } from "~/services/constants.server";
import { authenticator } from '~/services/auth.server';

export const loader = async ({ request, params }: LoaderFunctionArgs) => {
  invariant(params.groupId, "Missing groupId param");
  const user = await authenticator.isAuthenticated(request, {
    failureRedirect: '/',
  });
  const client = new GraphQLClient(API_URL, { headers: { authorization: `Bearer ${user.token}` } })
  const group = await client.request(GetGroupDetailQuery, { id: params.groupId })
  if (!group.getGroup) {
    throw new Response("Not Found", { status: 404 });
  }
  return json({ group });
};
