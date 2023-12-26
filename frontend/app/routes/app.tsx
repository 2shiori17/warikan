import type { LoaderFunction } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import { authenticator, User } from '~/services/auth.server';

import { Layout } from '~/components/layout';

export const loader: LoaderFunction = async ({ request }) => {
  const user = await authenticator.isAuthenticated(request, {
    failureRedirect: '/',
  });
  return { user };
};

export default function App() {
  const data = useLoaderData<{ user: User }>();

  return (
    <Layout>
      {data.user.id}
    </Layout>
  );
}
