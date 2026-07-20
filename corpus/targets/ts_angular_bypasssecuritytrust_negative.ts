// SAFE: user input sanitized with DOMPurify before bypassing security trust

import { Component, Pipe, PipeTransform } from '@angular/core'
import { DomSanitizer, SafeHtml } from '@angular/platform-browser'
import DOMPurify from 'dompurify'

@Pipe({ name: 'safeHtml' })
export class SafeHtmlPipe implements PipeTransform {
  constructor(private sanitizer: DomSanitizer) {}

  transform(value: string): SafeHtml {
    const clean = DOMPurify.sanitize(value)
    return this.sanitizer.bypassSecurityTrustHtml(clean)
  }
}

@Component({
  selector: 'app-comment',
  template: `<div [innerHTML]="comment.body | safeHtml"></div>`
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
