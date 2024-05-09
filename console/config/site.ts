export type SiteConfig = typeof siteConfig

export const siteConfig = {
  name: "console@next-hat.com",
  description: "The official NextHat Console",
  mainNav: [
    {
      title: "Home",
      href: "/",
    },
    {
      title: "Users",
      href: "/users",
    },
  ],
  links: {
    twitter: "https://twitter.com/shadcn",
    github: "https://github.com/shadcn/ui",
    docs: "https://ui.shadcn.com",
  },
}
