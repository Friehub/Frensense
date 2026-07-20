// [frensense]
// observation: v-html directive is used with a value derived from user input or URL params
// impact: attacker-controlled HTML is rendered in the user's browser, enabling XSS
// improvement: use v-text or mustache interpolation {{ }} for text, or sanitize HTML with DOMPurify

import { defineComponent, ref } from 'vue'

export default defineComponent({
  setup() {
    const userContent = ref('')

    function loadContent(id: string) {
      fetch(`/api/content/${id}`)
        .then(res => res.json())
        .then(data => {
          userContent.value = data.body
        })
    }

    return { userContent, loadContent }
  },
  template: `<div v-html="userContent"></div>`
})
