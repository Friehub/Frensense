// SAFE: template is static, user data is passed as prop

import { defineComponent, computed } from 'vue'

export default defineComponent({
  props: {
    userDisplay: { type: String, required: true }
  },
  setup(props) {
    const safeContent = computed(() => {
      return props.userDisplay.replace(/</g, '&lt;').replace(/>/g, '&gt;')
    })

    return { safeContent }
  },
  template: `<div>{{ safeContent }}</div>`
})
