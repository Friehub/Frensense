// SAFE: Uses predefined CSS class names instead of user-controlled CSS strings
interface UserThemeProps {
  theme: "light" | "dark" | "high-contrast";
}

const themeStyles: Record<string, string> = {
  light: "theme-light",
  dark: "theme-dark",
  "high-contrast": "theme-high-contrast",
};

export function UserTheme({ theme }: UserThemeProps) {
  const className = themeStyles[theme] ?? "theme-light";
  return (
    <>
      <style>{`.${className} { color: inherit; }`}</style>
      <div className={className}>Themed content</div>
    </>
  );
}
