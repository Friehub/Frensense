// [frensense]
// observation: Vue template string is constructed from user-controlled input and compiled at runtime
// impact: attacker can inject arbitrary Vue directives (v-on, v-bind) leading to XSS or data exfiltration
// improvement: never compile user-controlled template strings; use render functions with sanitized data

import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    templateProp: { type: String, required: true }
  },
  setup(props) {
    function buildRenderer() {
      const dynamicTemplate = `<div>{{ ${props.templateProp} }}</div>`
      return {
        template: dynamicTemplate
      }
    }

    return { buildRenderer }
  }
})
