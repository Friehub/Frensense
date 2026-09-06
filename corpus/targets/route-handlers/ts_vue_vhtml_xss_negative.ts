// SAFE: v-html replaced with text interpolation after sanitization

import { defineComponent, ref } from 'vue'
import DOMPurify from 'dompurify'

export default defineComponent({
  setup() {
    const userContent = ref('')

    function loadContent(id: string) {
      fetch(`/api/content/${id}`)
        .then(res => res.json())
        .then(data => {
          userContent.value = DOMPurify.sanitize(data.body)
        })
    }

    return { userContent, loadContent }
  },
  template: `<div>{{ userContent }}</div>`
})
