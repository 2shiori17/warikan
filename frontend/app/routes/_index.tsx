import type { LoaderFunction } from "@remix-run/node";
import { useLoaderData, Form } from "@remix-run/react";
import { authenticator, User } from '~/services/auth.server';

export const loader: LoaderFunction = async ({ request }) => {
  const user = await authenticator.isAuthenticated(request, {
    failureRedirect: '/login',
  });
  return { user };
};

export default function Index() {
  const data = useLoaderData<{ user: User }>();

  return (
    <div>
      {data.user && (
        <>
          <Form action="/logout" method="post">
            <button>Logout</button>
          </Form>
          <h1>{data.user.id}</h1>
        </>
      )}
      <h1>Welcome to Remix</h1>
    </div>
  );
}
