import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "GenSense",
  description: "Experimental Semantic Diagnostic Engine",
  themeConfig: {
    logo: '/logo.svg', // Placeholder for potential future logo
    nav: [
      { text: 'Home', link: '/' },
      { text: 'API Reference', link: '/api' },
      { text: 'Rule Catalog', link: '/rules' }
    ],
    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Introduction', link: '/' },
          { text: 'API Reference', link: '/api' },
        ]
      },
      {
        text: 'Rules',
        items: [
          { text: 'Rule Catalog', link: '/rules' }
        ]
      }
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Friehub/gensense' }
    ],
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2026 Friehub'
    }
  }
})
