import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "Frensense",
  description: "Compositional Taint Analysis Engine for Rust, TypeScript, and more.",
  cleanUrls: true,
  base: '/frensense/',
  themeConfig: {
    logo: '/logo.svg',
    nav: [
      { text: 'Guide', link: '/guide' },
      { text: 'API', link: '/api' },
      { text: 'Corpus', link: '/corpus' },
      { text: 'MCP', link: '/mcp' },
      { text: 'Editor', link: '/editor' },
      { text: 'Changelog', link: '/changelog' },
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
          { text: 'The Corpus (.frc)', link: '/corpus' },
        ]
      },
      {
        text: 'Integration',
        items: [
          { text: 'Editor Integration', link: '/editor' },
          { text: 'Extending Frensense', link: '/extending' },
          { text: 'MCP Server', link: '/mcp' },
        ]
      },
      {
        text: 'About',
        items: [
          { text: 'References & Acknowledgments', link: '/references' },
        ]
      },
      {
        text: 'Releases',
        items: [
          { text: 'Changelog', link: '/changelog' },
        ]
      }
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Friehub/frensense' }
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
