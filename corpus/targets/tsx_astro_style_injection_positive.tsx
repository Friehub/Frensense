// [frensense]
// observation: Astro component uses user-controlled CSS in a <style> tag, allowing data exfiltration via CSS selectors.
// impact: Attacker can inject CSS keylogger rules (@import url(...)) or attribute selectors to exfiltrate CSRF tokens and other sensitive data through CSS.
// improvement: Never interpolate user input into style tags. Use scoped CSS classes or CSS-in-JS with sanitization.

interface UserThemeProps {
  userCss: string;
}

export function UserTheme({ userCss }: UserThemeProps) {
  return (
    <>
      <style>{userCss}</style>
      <div className="content">Themed content</div>
    </>
  );
}
