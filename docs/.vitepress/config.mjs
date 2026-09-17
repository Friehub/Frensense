import { defineConfig } from 'vitepress'
import { withMermaid } from 'vitepress-plugin-mermaid'

export default withMermaid(defineConfig({
  title: "Friehub",
  description: "Deterministic Security Engine.",
  cleanUrls: true,
  head: [
    ['link', { rel: 'icon', type: 'image/png', href: '/favicon.png' }],
    ['link', { rel: 'preconnect', href: 'https://fonts.googleapis.com' }],
    ['link', { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' }],
    ['link', { href: 'https://fonts.googleapis.com/css2?family=Urbanist:wght@400;500;600;700;800&display=swap', rel: 'stylesheet' }]
  ],
  vite: {
    optimizeDeps: {
      include: [
        "fastdom", 
        "fastdom/extensions/fastdom-promised.js"
      ]
    }
  },
  markdown: {
    math: true
  },
  themeConfig: {
    nav: [
      { text: 'Paper', link: '/frensense-paper/00_OVERVIEW' },
      { text: 'Blog', link: '/blog/' }
    ],
    sidebar: {
      '/frensense-paper/': [
        {
          text: 'Frensense Paper',
          items: [
            { text: 'Overview', link: '/frensense-paper/00_OVERVIEW' },
            { text: 'Crate: FRC', link: '/frensense-paper/02_CRATE_FRC' },
            { text: 'Crate: Lang', link: '/frensense-paper/03_CRATE_LANG' },
            { text: 'Crate: Engine', link: '/frensense-paper/04_CRATE_ENGINE' },
            { text: 'Crate: Bundler', link: '/frensense-paper/05_CRATE_BUNDLER' },
            { text: 'Crate: Providers', link: '/frensense-paper/06_CRATE_PROVIDERS' },
            { text: 'Crate: Runtime', link: '/frensense-paper/07_CRATE_RUNTIME' },
            { text: 'Crate: Root', link: '/frensense-paper/08_CRATE_ROOT' },
            { text: 'Pipeline', link: '/frensense-paper/09_PIPELINE' },
            { text: 'Theory Map', link: '/frensense-paper/10_THEORY_MAP' },
            { text: 'Data Structures', link: '/frensense-paper/11_DATA_STRUCTURES' },
            { text: 'Limitations', link: '/frensense-paper/12_LIMITATIONS' },
            { text: 'Benchmarks', link: '/frensense-paper/13_BENCHMARKS' },
            { text: 'References', link: '/frensense-paper/14_REFERENCES' }
          ]
        }
      ]
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Friehub' },
      { icon: 'x', link: 'https://x.com/friehub' },
      { icon: 'linkedin', link: 'https://www.linkedin.com/company/friehub/' }
    ],
    footer: {
      message: 'Frensense is built and maintained by Friehub.',
      copyright: 'Copyright © 2026 Friehub'
    }
  }
}))
