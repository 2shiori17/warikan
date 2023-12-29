import { Link } from "@remix-run/react";
import { HomeIcon, BellIcon } from "@radix-ui/react-icons"

import { Button } from "~/components/ui/button"
import { Avatar, AvatarFallback, AvatarImage } from "~/components/ui/avatar"

export function Navbar() {
  return (
    <nav className="flex flex-row justify-end items-center space-x-2 h-12 border-b px-2">
      <Button asChild variant="ghost" size="icon">
        <Link to="/app/groups">
          <HomeIcon className="h-6 w-6" />
        </Link>
      </Button>
      <Button asChild variant="ghost" size="icon">
        <Link to="/app/notifications">
          <BellIcon className="h-6 w-6" />
        </Link>
      </Button>
      <Link to="/app/settings">
        <Avatar>
          <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
          <AvatarFallback>CN</AvatarFallback>
        </Avatar>
      </Link>
    </nav>
  )
}
