// [frensense]
// observation: Recharts Tooltip renders user-controlled data via the label formatter without sanitization, enabling XSS.
// impact: Attacker-controlled data displayed in tooltips executes arbitrary JavaScript via dangerouslySetInnerHTML or raw HTML rendering.
// improvement: Sanitize tooltip label/values with DOMPurify or use React's text rendering instead of HTML.

import { Tooltip, LineChart, Line, XAxis, YAxis } from "recharts";

interface DataPoint {
  name: string;
  value: number;
}

interface ChartProps {
  data: DataPoint[];
}

export function VulnerabilityChart({ data }: ChartProps) {
  return (
    <LineChart data={data}>
      <XAxis dataKey="name" />
      <YAxis />
      <Tooltip contentStyle={{ color: "black" }} />
      <Line type="monotone" dataKey="value" />
    </LineChart>
  );
}
