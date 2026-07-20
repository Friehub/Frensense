// SAFE: render function used with h() instead of template string

import { defineComponent, h } from 'vue'

export default defineComponent({
  props: {
    userDisplay: { type: String, required: true }
  },
  setup(props) {
    return () => h('div', props.userDisplay)
  }
})
