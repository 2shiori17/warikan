import type { LoaderFunctionArgs } from "@remix-run/node";
import { json } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import invariant from "tiny-invariant";

import { getGroup } from "~/lib/data";
import { H3 } from "~/components/typography";
import { GroupCard } from "~/components/group";
import { PaymentCardList } from "~/components/payment";

export const loader = async ({ params }: LoaderFunctionArgs) => {
  invariant(params.groupId, "Missing groupId param");
  const group = await getGroup(params.groupId);
  if (!group) {
    throw new Response("Not Found", { status: 404 });
  }
  return json({ group });
};

export default function GroupDetail() {
  const { group } = useLoaderData<typeof loader>();

  return (
    <div>
      <GroupCard group={group} />
      <div>
        <H3 className="pt-4 pb-2">Payments</H3>
        <PaymentCardList payments={group.payments} />
      </div>
    </div>
  )
}
