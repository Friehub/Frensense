import { createContentLoader } from 'vitepress'

export default createContentLoader('blog/*.md', {
  transform(raw) {
    return raw
      .filter(({ url }) => url !== '/blog/')
      .map(({ url, frontmatter }) => ({
        title: frontmatter.title || 'Untitled',
        url,
        date: frontmatter.date || '',
        excerpt: frontmatter.excerpt || ''
      }))
      .sort((a, b) => new Date(b.date) - new Date(a.date))
  }
})
