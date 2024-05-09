"use client"

import { DropdownMenuGroup } from "@radix-ui/react-dropdown-menu"

import type { User } from "@/types/user"
import { siteConfig } from "@/config/site"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { MainNav } from "@/components/main-nav"

export interface AvatarMenuProps {
  user: User
}

export function AvatarMenu({ user }: AvatarMenuProps) {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild className="outline-none">
        <Avatar className="outline-none">
          <AvatarImage src="" alt="@leone" />
          <AvatarFallback>L</AvatarFallback>
        </Avatar>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end" className="relative right-[20px] w-56">
        <DropdownMenuGroup>
          <DropdownMenuItem className="flex-col items-start">
            <p className="pb-1 text-sm font-medium leading-none">{user.name}</p>
            <p className="text-xs leading-none text-muted-foreground">
              {user.email}
            </p>
          </DropdownMenuItem>
        </DropdownMenuGroup>
        <DropdownMenuSeparator />
        <DropdownMenuGroup>
          <DropdownMenuItem>Billing</DropdownMenuItem>
          <DropdownMenuItem>Team</DropdownMenuItem>
          <DropdownMenuItem>Subscription</DropdownMenuItem>
        </DropdownMenuGroup>
        <DropdownMenuSeparator />
        <DropdownMenuGroup>
          <DropdownMenuItem>Logout</DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}

export interface SiteHeaderProps {
  user: User
}

export function SiteHeader({ user }: SiteHeaderProps) {
  return (
    <header className="bg-secondary-background sticky top-0 z-40 w-full border-b">
      <div className="container flex h-16 items-center space-x-4 sm:justify-between sm:space-x-0">
        <MainNav items={siteConfig.mainNav} />
        <div className="flex flex-1 items-center justify-end space-x-4">
          <nav className="flex items-center space-x-1">
            <AvatarMenu user={user} />
          </nav>
        </div>
      </div>
    </header>
  )
}
