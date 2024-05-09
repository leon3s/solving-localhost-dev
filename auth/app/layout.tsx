import "@/styles/globals.css"
import { headers } from "next/headers"

export const metadata = {
  title: "Sign in · NextHat",
  description: "Sign in to Next Hat",
}

export default async function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  const headersList = headers()
  const referer = headersList.get("referer")

  console.log(referer)

  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
