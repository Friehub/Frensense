// SAFE: Disables JavaScript in WebView for untrusted content, preventing XSS
import { WebView } from "react-native-webview";

interface WebViewProps {
  userHtml: string;
}

export function UserContentWebView({ userHtml }: WebViewProps) {
  return (
    <WebView
      source={{ html: userHtml }}
      javaScriptEnabled={false}
      domStorageEnabled={false}
      style={{ flex: 1 }}
    />
  );
}
