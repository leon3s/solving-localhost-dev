import "@/styles/globals.css"

export const metadata = {
  title: "Sign in · NextHat",
  description: "Sign in to Next Hat",
}

console.log("layout !")

export default async function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
