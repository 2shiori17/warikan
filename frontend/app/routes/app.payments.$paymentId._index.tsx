import type { LoaderFunctionArgs } from "@remix-run/node";
import { json } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import invariant from "tiny-invariant";

import { getPayment } from "~/lib/data";
import { PaymentCard } from "~/components/payment";

export const loader = async ({ params }: LoaderFunctionArgs) => {
  invariant(params.paymentId, "Missing paymentId param");
  const payment = await getPayment(params.paymentId);
  if (!payment) {
    throw new Response("Not Found", { status: 404 });
  }
  return json({ payment });
};

export default function GroupDetail() {
  const { payment } = useLoaderData<typeof loader>();

  return (
    <div>
      <PaymentCard payment={payment} />
    </div>
  )
}
