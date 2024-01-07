import { LoaderFunctionArgs, json } from "@remix-run/node";
import { Link, useLoaderData } from "@remix-run/react";
import { CalendarIcon, PersonIcon } from '@radix-ui/react-icons'
import { format, compareDesc } from "date-fns";
import { GraphQLClient } from "graphql-request";

import { Card, CardHeader, CardTitle, CardDescription } from "~/components/ui/card";
import { GetGroupsByUserQuery } from "~/lib/data";
import { authenticator } from '~/services/auth.server';
import { API_URL } from "~/services/constants.server";

export const loader = async ({ request }: LoaderFunctionArgs) => {
  const user = await authenticator.isAuthenticated(request, {
    failureRedirect: '/',
  });
  const client = new GraphQLClient(API_URL, { headers: { authorization: `Bearer ${user.token}` } })
  const groups = await client.request(GetGroupsByUserQuery)
  return json({ groups });
};

export default function GroupList() {
  const { groups } = useLoaderData<typeof loader>();

  return (
    <div className="flex flex-col space-y-2">
      {
        groups.getGroupsByUser
          .sort((a, b) => compareDesc(a.createdAt, b.createdAt))
          .map((group) => (
            <Link key={group.id} to={`/app/groups/${group.id}`}>
              <Card>
                <CardHeader>
                  <CardTitle>
                    {group.title}
                  </CardTitle>
                  <CardDescription>
                    <span className="inline-flex items-baseline mr-2">
                      <CalendarIcon className="self-center w-4 h-4 mx-1" />
                      <span>{format(group.createdAt, "yyyy/MM/dd")}</span>
                    </span>
                    <span className="inline-flex items-baseline">
                      <PersonIcon className="self-center w-4 h-4 mx-1" />
                      <span>{group.participants.length}人</span>
                    </span>
                  </CardDescription>
                </CardHeader>
              </Card>
            </Link>
          ))
      }
    </div>
  );
}
