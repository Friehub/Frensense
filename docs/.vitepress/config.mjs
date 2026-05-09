import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "GenSense",
  description: "Experimental Semantic Diagnostic Engine for Rust, TypeScript, and Solidity.",
  cleanUrls: true,
  themeConfig: {
    logo: '/logo.svg',
    nav: [
      { text: 'Guide', link: '/guide' },
      { text: 'API', link: '/api' },
      { text: 'Rules', link: '/rules' },
      { text: 'Editor', link: '/editor' },
    ],
    sidebar: [
      {
        text: 'Introduction',
        items: [
          { text: 'Overview', link: '/' },
          { text: 'Getting Started', link: '/guide' },
        ]
      },
      {
        text: 'Reference',
        items: [
          { text: 'API Reference', link: '/api' },
          { text: 'Rule Catalog', link: '/rules' },
        ]
      },
      {
        text: 'Integration',
        items: [
          { text: 'Editor Integration', link: '/editor' },
          { text: 'Extending GenSense', link: '/extending' },
        ]
      }
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Friehub/gensense' }
    ],
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2026 Friehub'
    },
    search: {
      provider: 'local'
    }
  }
})
