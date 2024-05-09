"use client"

import React from "react"
import { store } from "@/store"
import { setMe } from "@/store/me-slice"

import type { User } from "@/types/user"

export interface PreloaderProps {
  user: User
}

export function Preloader({ user }: PreloaderProps) {
  const loaded = React.useRef(false)
  if (!loaded.current) {
    store.dispatch(setMe(user))
    loaded.current = true
  }
  return null
}
