export function decodeFileContent(data: { content: string, encoding: string }) {
  if (data.encoding === "base64") {
    // Correct: manually decode UTF-8 bytes to properly handle emojis/accents
    data.content = new TextDecoder().decode(Uint8Array.from(atob(data.content), c => c.charCodeAt(0)));
  }
  return data.content;
}
