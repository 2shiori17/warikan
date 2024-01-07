import type { LoaderFunctionArgs } from "@remix-run/node";
import { json } from "@remix-run/node";
import { Link, useLoaderData, useRouteLoaderData } from "@remix-run/react";
import { CalendarIcon, PersonIcon } from '@radix-ui/react-icons'
import { format } from "date-fns";
import invariant from "tiny-invariant"

import { Card, CardHeader, CardTitle, CardDescription } from "~/components/ui/card";
import { loader as routeLoader } from "./app.groups.$groupId";

export const loader = async ({ params }: LoaderFunctionArgs) => {
  invariant(params.paymentId, "Missing paymentId param");
  return json({ paymentId: params.paymentId });
};

export default function GroupDetail() {
  const { paymentId } = useLoaderData<typeof loader>();
  const routeData = useRouteLoaderData<typeof routeLoader>("routes/app.groups.$groupId");
  invariant(routeData, "Missing data");
  invariant(routeData.group.getGroup, "Missing data");
  const group = routeData.group.getGroup;
  const payment = group.payments.find((payment) => payment.id === paymentId);
  invariant(payment, "Missing data");

  return (
    <div>
      <Link to={`/app/groups/${group.id}/payments/${payment.id}`}>
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
    </div>
  )
}
