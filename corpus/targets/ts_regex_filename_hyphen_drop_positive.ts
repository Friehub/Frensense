export function parseGeneratedFiles(text: string) {
  // Regex drops hyphenated filenames
  const fileRegex = /---\s*([^\s-]+)\s*---([\s\S]*?)(?=(?:---\s*[^\s-]+\s*---|$))/g;
  const files = [];
  let match;
  
  while ((match = fileRegex.exec(text)) !== null) {
    files.push({
      path: match[1],
      content: match[2].trim()
    });
  }
  
  return files;
}
