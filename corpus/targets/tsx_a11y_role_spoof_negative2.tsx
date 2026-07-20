// SAFE: Retains <div> but adds proper keyboard handling for correct accessibility semantics
interface ClickableCardProps {
  onClick: () => void;
  children: React.ReactNode;
}

export function ClickableCard({ onClick, children }: ClickableCardProps) {
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" || e.key === " ") {
      onClick();
    }
  };
  return (
    <div role="button" tabIndex={0} onClick={onClick} onKeyDown={handleKeyDown}>
      {children}
    </div>
  );
}
