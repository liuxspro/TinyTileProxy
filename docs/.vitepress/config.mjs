import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  base: "/docs/",
  title: "Tiny Tile Proxy",
  description: "A Simple Tile Porxy Server",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [{ text: "Home", link: "/" }],

    sidebar: [
      {
        text: "Ueage",
        items: [
          { text: "Proxys", link: "/proxys" },
          { text: "Setting", link: "/setting" },
        ],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/liuxspro/TinyTileProxy" },
    ],
  },
  ignoreDeadLinks: true,
  head: [["link", { rel: "icon", href: "/docs/favicon.ico" }]],
});
