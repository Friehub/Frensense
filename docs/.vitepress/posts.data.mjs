import { createContentLoader } from 'vitepress'

export default createContentLoader('blog/*.md', {
  transform(raw) {
    return raw
      .filter(({ url }) => url !== '/blog/')
      .map(({ url, frontmatter }) => {
        let formattedDate = '';
        if (frontmatter.date) {
          const d = new Date(frontmatter.date);
          d.setUTCHours(12); // Prevent timezone shifting to previous day
          formattedDate = d.toLocaleDateString('en-US', { 
            year: 'numeric', 
            month: 'short', 
            day: 'numeric' 
          });
        }
        return {
          title: frontmatter.title || 'Untitled',
          url,
          date: formattedDate,
          rawDate: frontmatter.date,
          excerpt: frontmatter.excerpt || ''
        }
      })
      .sort((a, b) => new Date(b.rawDate) - new Date(a.rawDate))
  }
})
