// SAFE: Uses native <button> element which provides correct semantics and keyboard interaction
interface ClickableCardProps {
  onClick: () => void;
  children: React.ReactNode;
}

export function ClickableCard({ onClick, children }: ClickableCardProps) {
  return (
    <button type="button" onClick={onClick}>
      {children}
    </button>
  );
}
