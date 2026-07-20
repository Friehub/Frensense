// SAFE: custom sanitizer still sanitizes via DOMPurify instead of raw pass-through

import { NgModule } from '@angular/core'
import { BrowserModule, DomSanitizer, type SafeHtml } from '@angular/platform-browser'
import { AppComponent } from './app.component'
import DOMPurify from 'dompurify'

export class SafeCustomSanitizer extends DomSanitizer {
  sanitize(context: any, value: string): string {
    return DOMPurify.sanitize(value)
  }

  bypassSecurityTrustHtml(value: string): SafeHtml {
    return DOMPurify.sanitize(value) as any
  }

  bypassSecurityTrustStyle(value: string): any { return value }
  bypassSecurityTrustScript(value: string): any { return value }
  bypassSecurityTrustUrl(value: string): any { return value }
  bypassSecurityTrustResourceUrl(value: string): any { return value }
}

@NgModule({
  imports: [BrowserModule],
  declarations: [AppComponent],
  providers: [{ provide: DomSanitizer, useClass: SafeCustomSanitizer }],
  bootstrap: [AppComponent]
})
export class AppModule {}
