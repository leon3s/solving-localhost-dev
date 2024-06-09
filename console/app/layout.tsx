import React from "react"
import { Metadata } from "next"
import { cookies } from "next/headers"
import { redirect } from "next/navigation"

import "@/styles/globals.css"
import { store } from "@/store"

// import { Provider } from "react-redux"

import { siteConfig } from "@/config/site"
import axios from "@/lib/api"
import { fontSans } from "@/lib/fonts"
import { cn } from "@/lib/utils"
import { Preloader } from "@/components/preloader"
import { Provider } from "@/components/provider"
import { SiteHeader } from "@/components/site-header"
import { TailwindIndicator } from "@/components/tailwind-indicator"
import { ThemeProvider } from "@/components/theme-provider"

export const metadata: Metadata = {
  title: {
    default: "Console · NextHat",
    template: `%s · NextHat`,
  },
  description: siteConfig.description,
  themeColor: [
    { media: "(prefers-color-scheme: light)", color: "white" },
    { media: "(prefers-color-scheme: dark)", color: "black" },
  ],
  icons: {
    icon: "/favicon.ico",
    shortcut: "/favicon-16x16.png",
    apple: "/apple-touch-icon.png",
  },
}

interface RootLayoutProps {
  children: React.ReactNode
  params: {
    user: any
  }
}

export default async function RootLayout({
  children,
  params,
}: RootLayoutProps) {
  const cookieStore = cookies()

  const cookie = cookieStore.get("nhtiam")

  if (cookie === undefined) {
    return redirect(
      `${process.env.AUTH_URL}?redirect=${process.env.APP_URL}`
    )
  }

  try {
    const res = await axios.get("/users/me", {
      headers: {
        Cookie: `nhtiam=${cookie.value};`,
      },
    })
    console.log(res)
    params.user = res.data
  } catch (err) {
    console.log(err)
    return redirect(
      `${process.env.AUTH_URL}?redirect=http://console.dev.next-hat.com`
    )
  }

  return (
    <>
      <html lang="en" suppressHydrationWarning>
        <head />
        <body
          className={cn(
            "min-h-screen bg-background font-sans antialiased",
            fontSans.variable
          )}
        >
          <Preloader user={params.user} />
          <Provider>
            <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
              <div className="relative flex min-h-screen flex-col">
                <SiteHeader user={params.user} />
                <div className="flex-1">{children}</div>
              </div>
              <TailwindIndicator />
            </ThemeProvider>
          </Provider>
        </body>
      </html>
    </>
  )
}
