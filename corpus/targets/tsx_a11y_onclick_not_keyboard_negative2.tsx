// SAFE: Retains <span> but adds proper keyboard event handler for accessibility
interface NavItemProps {
  label: string;
  onNavigate: () => void;
}

export function NavItem({ label, onNavigate }: NavItemProps) {
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      onNavigate();
    }
  };
  return (
    <span role="link" tabIndex={0} onClick={onNavigate} onKeyDown={handleKeyDown}>
      {label}
    </span>
  );
}
