import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "Frensense",
  description: "Deterministic Security & Diagnostics.",
  cleanUrls: true,
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
      { icon: 'github', link: 'https://github.com/Friehub/Frensense' }
    ]
  }
})
