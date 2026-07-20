// [frensense]
// observation: DomSanitizer.bypassSecurityTrustHtml is called with user-controlled input
// impact: attacker-controlled HTML bypasses Angular's built-in sanitization, enabling XSS
// improvement: use bypassSecurityTrustHtml only for trusted content, sanitize user input with DOMPurify first

import { Component, Pipe, PipeTransform } from '@angular/core'
import { DomSanitizer, SafeHtml } from '@angular/platform-browser'

@Pipe({ name: 'safeHtml' })
export class SafeHtmlPipe implements PipeTransform {
  constructor(private sanitizer: DomSanitizer) {}

  transform(value: string): SafeHtml {
    return this.sanitizer.bypassSecurityTrustHtml(value)
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
