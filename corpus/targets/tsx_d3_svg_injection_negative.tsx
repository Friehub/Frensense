// SAFE: Uses React JSX for SVG elements instead of D3 .attr() with user data, leveraging React's built-in XSS protection
interface SvgProps {
  userLabel: string;
}

export function DynamicSvg({ userLabel }: SvgProps) {
  return (
    <svg width={200} height={100}>
      <text x={10} y={20}>{userLabel}</text>
    </svg>
  );
}
