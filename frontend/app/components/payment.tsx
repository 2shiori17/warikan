import { Link } from "@remix-run/react";
import { CalendarIcon, PersonIcon } from '@radix-ui/react-icons'
import { format, compareDesc } from "date-fns";

import { Payment } from "~/lib/data"
import { Card, CardHeader, CardTitle, CardDescription } from "~/components/ui/card";

export interface PaymentCardProps {
  group_id: string,
  payment: Payment,
}

export function PaymentCard(props: PaymentCardProps) {
  return (
    <Link to={`/app/groups/${props.group_id}/payments/${props.payment.id}`}>
      <Card>
        <CardHeader>
          <CardTitle>
            {props.payment.name}
          </CardTitle>
          <CardDescription>
            <span className="inline-flex items-baseline mr-2">
              <CalendarIcon className="self-center w-4 h-4 mx-1" />
              <span>{format(props.payment.created_at, "yyyy/MM/dd")}</span>
            </span>
            <span className="inline-flex items-baseline">
              <PersonIcon className="self-center w-4 h-4 mx-1" />
              <span>{props.payment.creditor.name}</span>
            </span>
          </CardDescription>
        </CardHeader>
      </Card>
    </Link>
  )
}

export interface PaymentCardListProps {
  group_id: string,
  payments: Payment[],
}

export function PaymentCardList(props: PaymentCardListProps) {
  return (
    <div className="flex flex-col space-y-2">
      {
        props.payments
          .sort((a, b) => compareDesc(a.created_at, b.created_at))
          .map((payment) => <PaymentCard key={payment.id} group_id={props.group_id} payment={payment} />)
      }
    </div>
  )
}
