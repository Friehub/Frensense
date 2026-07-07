export function decodeFileContent(data: { content: string, encoding: string }) {
  if (data.encoding === "base64") {
    // Incorrect: atob decodes to Latin-1, corrupting UTF-8 characters
    data.content = atob(data.content);
  }
  return data.content;
}
