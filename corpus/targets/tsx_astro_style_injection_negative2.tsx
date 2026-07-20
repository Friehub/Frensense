// SAFE: Uses CSS custom properties from a preset object, not user-provided CSS strings
interface UserThemeProps {
  primaryColor: string;
  bgColor: string;
}

const VALID_COLORS = new Set(["red", "blue", "green", "purple", "orange"]);

export function UserTheme({ primaryColor, bgColor }: UserThemeProps) {
  const safePrimary = VALID_COLORS.has(primaryColor) ? primaryColor : "blue";
  const safeBg = VALID_COLORS.has(bgColor) ? bgColor : "white";
  return (
    <>
      <style>{`:root { --primary: ${safePrimary}; --bg: ${safeBg}; }`}</style>
      <div style={{ color: `var(--primary)`, background: `var(--bg)` }}>Content</div>
    </>
  );
}
