import { BellIcon } from "@radix-ui/react-icons"

import { Button } from "~/components/ui/button"
import { Avatar, AvatarFallback, AvatarImage } from "~/components/ui/avatar"

export function Navbar() {
  return (
    <nav className="flex flex-row justify-end items-center space-x-2 h-12 border-b px-2">
      <Button variant="ghost" size="icon">
        <BellIcon className="h-6 w-6" />
      </Button>
      <Avatar>
        <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
        <AvatarFallback>CN</AvatarFallback>
      </Avatar>
    </nav>
  )
}
