import { useRouteLoaderData, Link } from "@remix-run/react";
import { CalendarIcon, PersonIcon } from '@radix-ui/react-icons'
import { format, compareDesc } from "date-fns";
import invariant from "tiny-invariant";

import { Card, CardHeader, CardTitle, CardDescription } from "~/components/ui/card";
import { H3 } from "~/components/typography";
import { loader as routeLoader } from "./app.groups.$groupId";

export default function GroupDetail() {
  const data = useRouteLoaderData<typeof routeLoader>("routes/app.groups.$groupId");
  invariant(data, "Missing data");
  invariant(data.group.getGroup, "Missing data");
  const group = data.group.getGroup;

  return (
    <div>
      <Link to={`/app/groups/${group.id}`}>
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
      <div>
        <H3 className="pt-4 pb-2">Payments</H3>
        <div className="flex flex-col space-y-2">
          {
            group.payments
              .sort((a, b) => compareDesc(a.createdAt, b.createdAt))
              .map((payment) => (
                <Link key={payment.id} to={`/app/groups/${group.id}/payments/${payment.id}`}>
                  <Card>
                    <CardHeader>
                      <CardTitle>
                        {payment.title}
                      </CardTitle>
                      <CardDescription>
                        <span className="inline-flex items-baseline mr-2">
                          <CalendarIcon className="self-center w-4 h-4 mx-1" />
                          <span>{format(payment.createdAt, "yyyy/MM/dd")}</span>
                        </span>
                        <span className="inline-flex items-baseline">
                          <PersonIcon className="self-center w-4 h-4 mx-1" />
                          <span>{payment.creditor.name}</span>
                        </span>
                      </CardDescription>
                    </CardHeader>
                  </Card>
                </Link>
              ))
          }
        </div>
      </div>
    </div>
  )
}
