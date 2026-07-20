// SAFE: Uses split/filter parsing instead of regex to avoid character class issues
export function parseGeneratedFiles(text: string) {
  const sections = text.split(/---\s*/).filter(Boolean);
  const files = [];
  for (let i = 0; i + 1 < sections.length; i += 2) {
    const path = sections[i].trim();
    const content = sections[i + 1].replace(/\s*---[\s\S]*$/, "").trim();
    files.push({ path, content });
  }
  return files;
}
