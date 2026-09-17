---
title: Frensense Blog
---

# Frensense Blog

<script setup>
import { data as posts } from '../.vitepress/posts.data.mjs'
</script>

<div class="blog-list">
  <div v-for="post in posts" :key="post.url" class="blog-post">
    <h2><a :href="post.url">{{ post.title }}</a></h2>
    <div class="date">{{ post.date }}</div>
    <p>{{ post.excerpt }}</p>
    <a :href="post.url" class="read-more">Read more →</a>
  </div>
</div>

<style>
.blog-list { margin-top: 2rem; }
.blog-post { 
  margin-bottom: 3rem; 
  padding-bottom: 2rem;
  border-bottom: 1px solid var(--vp-c-divider);
}
.blog-post h2 { margin-top: 0; margin-bottom: 0.5rem; border: none; padding: 0; }
.blog-post h2 a { color: var(--vp-c-text-1); text-decoration: none; }
.blog-post h2 a:hover { color: var(--vp-c-brand-1); }
.date { color: var(--vp-c-text-2); font-size: 0.9em; margin-bottom: 1rem; }
.read-more { color: var(--vp-c-brand-1); font-weight: 500; font-size: 0.9em; }
</style>
