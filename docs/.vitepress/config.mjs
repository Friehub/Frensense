import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "GenSense",
  description: "Experimental Semantic Diagnostic Engine for Rust, TypeScript, and Solidity.",
  cleanUrls: true,
  base: '/gensense/',
  themeConfig: {
    logo: '/logo.svg',
    nav: [
      { text: 'Guide', link: '/guide' },
      { text: 'API', link: '/api' },
      { text: 'Rules', link: '/rules' },
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
          { text: 'Rule Catalog', link: '/rules' },
          { text: 'Rule Authoring & Schemas', link: '/authoring' },
        ]
      },
      {
        text: 'Integration',
        items: [
          { text: 'Editor Integration', link: '/editor' },
          { text: 'Extending GenSense', link: '/extending' },
          { text: 'MCP Server', link: '/mcp' },
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
