// SAFE: Uses custom Tooltip component that escapes HTML entities in displayed values
import { Tooltip, LineChart, Line, XAxis, YAxis } from "recharts";

function escapeHtml(str: string): string {
  return str.replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

interface DataPoint {
  name: string;
  value: number;
}

interface ChartProps {
  data: DataPoint[];
}

function CustomTooltip({ active, payload, label }: { active?: boolean; payload?: { value: number }[]; label?: string }) {
  if (!active || !payload) return null;
  return (
    <div style={{ background: "white", padding: "8px" }}>
      <p>{escapeHtml(label ?? "")}</p>
      <p>{payload[0]?.value}</p>
    </div>
  );
}

export function VulnerabilityChart({ data }: ChartProps) {
  return (
    <LineChart data={data}>
      <XAxis dataKey="name" />
      <YAxis />
      <Tooltip content={<CustomTooltip />} />
      <Line type="monotone" dataKey="value" />
    </LineChart>
  );
}
