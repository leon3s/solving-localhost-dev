"use client"

import React from "react"
import { store } from "@/store"
import { Provider as ReduxProvider } from "react-redux"

export interface ProviderProps {
  children: React.ReactNode
}

export function Provider({ children }: ProviderProps) {
  return <ReduxProvider store={store}>{children}</ReduxProvider>
}
