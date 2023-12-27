import type { LoaderFunction } from "@remix-run/node";
import { Outlet } from "@remix-run/react";

import { authenticator } from '~/services/auth.server';
import { Layout } from '~/components/layout';

export const loader: LoaderFunction = async ({ request }) => {
  const user = await authenticator.isAuthenticated(request, {
    failureRedirect: '/',
  });
  return { user };
};

export default function App() {
  return (
    <Layout>
      <Outlet />
    </Layout>
  );
}
