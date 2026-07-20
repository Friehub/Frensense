// SAFE: Sanitizes user HTML with DOMPurify before rendering in WebView
import { WebView } from "react-native-webview";
import DOMPurify from "dompurify";

interface WebViewProps {
  userHtml: string;
}

export function UserContentWebView({ userHtml }: WebViewProps) {
  const sanitizedHtml = DOMPurify.sanitize(userHtml);
  return (
    <WebView
      source={{ html: sanitizedHtml }}
      style={{ flex: 1 }}
    />
  );
}
