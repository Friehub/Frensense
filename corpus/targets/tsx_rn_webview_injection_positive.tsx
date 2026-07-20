// [frensense]
// observation: React Native WebView renders user-controlled HTML directly via source.html property without sanitization, enabling XSS.
// impact: Attacker-controlled HTML executes arbitrary JavaScript in the WebView context, stealing tokens, accessing AsyncStorage, or making API calls.
// improvement: Sanitize HTML content with DOMPurify before passing to WebView, or use a sandboxed WebView with JavaScript disabled for untrusted content.

import { WebView } from "react-native-webview";

interface WebViewProps {
  userHtml: string;
}

export function UserContentWebView({ userHtml }: WebViewProps) {
  return (
    <WebView
      source={{ html: userHtml }}
      style={{ flex: 1 }}
    />
  );
}
