import { ReactNode } from 'react';

import { Navbar } from '~/components/navbar';
import { Container } from '~/components/container';
import { Copyright } from '~/components/copyright';

export interface LayoutProps {
  children?: ReactNode
}

export function Layout(props: LayoutProps) {
  return (
    <Container>
      <header>
        <Navbar />
      </header>
      <main className="my-4">
        {props.children}
      </main>
      <footer className="sticky top-[100vh]">
        <Copyright />
      </footer>
    </Container>
  )
}
