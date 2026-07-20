// SAFE: Uses native <a> tag which provides built-in keyboard activation
interface NavItemProps {
  label: string;
  onNavigate: () => void;
}

export function NavItem({ label, onNavigate }: NavItemProps) {
  return (
    <a href="#" onClick={(e) => { e.preventDefault(); onNavigate(); }}>
      {label}
    </a>
  );
}
