import * as path from "path";
import { defineConfig } from "rspress/config";

export default defineConfig({
  root: path.join(__dirname, "docs"),
  base: "/docs/",
  title: "Tiny Tile Proxy",
  description: "Rspack-based Static Site Generator",
  icon: "/icon.png",
  logo: {
    light: "/icon.png",
    dark: "/icon.png",
  },
  themeConfig: {
    socialLinks: [{ icon: "github", mode: "link", content: "https://github.com/web-infra-dev/rspress" }],
  },
});
