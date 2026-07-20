// SAFE: Sanitizes tooltip content with DOMPurify before rendering
import { Tooltip, LineChart, Line, XAxis, YAxis } from "recharts";
import DOMPurify from "dompurify";

interface DataPoint {
  name: string;
  value: number;
}

interface ChartProps {
  data: DataPoint[];
}

function SafeTooltip({ active, payload, label }: { active?: boolean; payload?: { value: number }[]; label?: string }) {
  if (!active || !payload) return null;
  const safeLabel = DOMPurify.sanitize(label ?? "");
  return (
    <div style={{ background: "white", padding: "8px" }}>
      <p>{safeLabel}</p>
      <p>{payload[0]?.value}</p>
    </div>
  );
}

export function VulnerabilityChart({ data }: ChartProps) {
  return (
    <LineChart data={data}>
      <XAxis dataKey="name" />
      <YAxis />
      <Tooltip content={<SafeTooltip />} />
      <Line type="monotone" dataKey="value" />
    </LineChart>
  );
}
