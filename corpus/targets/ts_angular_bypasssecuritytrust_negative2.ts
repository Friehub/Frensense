// SAFE: innerHtml not used; Angular template expression escapes automatically

import { Component } from '@angular/core'

@Component({
  selector: 'app-comment',
  template: `<div>{{ comment.body }}</div>`
})
export class CommentComponent {
  comment: { body: string } = { body: '' }

  loadComment(id: string) {
    fetch(`/api/comments/${id}`)
      .then(r => r.json())
      .then(data => {
        this.comment = data
      })
  }
}
