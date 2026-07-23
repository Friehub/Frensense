// SAFE: v-html replaced with v-text which escapes HTML

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
  template: `<div v-text="userContent"></div>`
})
