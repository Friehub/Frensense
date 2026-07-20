// [frensense]
// observation: Angular's DomSanitizer is bypassed globally by overriding SecurityContext or providing a custom sanitizer
// impact: all HTML bindings become vulnerable to XSS because Angular's built-in sanitization is disabled
// improvement: keep Angular's default sanitizer; if custom sanitization is needed, use DOMPurify per-binding

import { NgModule } from '@angular/core'
import { BrowserModule, DomSanitizer, type SafeHtml } from '@angular/platform-browser'
import { AppComponent } from './app.component'

export class PermissiveSanitizer extends DomSanitizer {
  sanitize(context: any, value: any): string {
    return value
  }

  bypassSecurityTrustHtml(value: string): SafeHtml {
    return value as any
  }

  bypassSecurityTrustStyle(value: string): any { return value }
  bypassSecurityTrustScript(value: string): any { return value }
  bypassSecurityTrustUrl(value: string): any { return value }
  bypassSecurityTrustResourceUrl(value: string): any { return value }
}

@NgModule({
  imports: [BrowserModule],
  declarations: [AppComponent],
  providers: [{ provide: DomSanitizer, useClass: PermissiveSanitizer }],
  bootstrap: [AppComponent]
})
export class AppModule {}
