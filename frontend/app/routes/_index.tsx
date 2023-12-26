import type { LoaderFunction } from "@remix-run/node";
import { Form } from "@remix-run/react";
import { authenticator } from '~/services/auth.server';

import { Button } from "~/components/ui/button"

export const loader: LoaderFunction = async ({ request }) => {
  const user = await authenticator.isAuthenticated(request, {
    successRedirect: "/app",
  });
  return { user };
};

export default function Index() {
  return (
    <div>
      <Form action="/auth/login" method="post">
        <Button>Login</Button>
      </Form>
    </div>
  );
}
